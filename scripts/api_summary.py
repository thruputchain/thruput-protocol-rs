"""Summarizes a crate's public API from its rustdoc JSON, one item per line.

Usage: python3 api_summary.py [--docs] [--attrs] CRATE.json

Each line describes a public item: a field and its type, an enum variant
with its discriminant and fields, a constant with its type and value, a
trait impl with each of its items' signatures, a function, a type alias or a
trait method. Lines are sorted, so two summaries can be compared with diff.
Ids and spans, which differ between builds, are left out.

--docs   also a line with the doc comment of each item, field, variant,
         impl and impl member that has one
--attrs  also a line with the attributes rustdoc records for each of those,
         with the source spans in them left out

compare-xdrc-api.sh uses this.
"""

import argparse
import json
import re


def strip(value):
    """Drops the parts of rustdoc JSON that differ between builds."""
    if isinstance(value, dict):
        return {
            k: strip(v)
            for k, v in value.items()
            if k not in ("id", "span", "docs", "links", "attrs")
        }
    if isinstance(value, list):
        return [strip(v) for v in value]
    return value


def show(value):
    return json.dumps(strip(value), sort_keys=True)


# The source span rustc records in some attributes, such as a `cfg_attr`'s
# `CfgTrace`: `span: path/to/file.rs:57:7: 57:23 (#0)`.
SPAN = re.compile(r"span: \S+:\d+:\d+: \d+:\d+ \(#\d+\)")


def attr(a):
    """An attribute, with any source span in it left out."""
    if isinstance(a, dict) and isinstance(a.get("other"), str):
        return {"other": SPAN.sub("span: _", a["other"])}
    return a


def summarize(doc, docs=False, attrs=False):
    index, paths = doc["index"], doc["paths"]
    out = set()

    def meta(label, item):
        """The doc comment and attributes of `item`, if asked for."""
        if docs and item.get("docs"):
            out.add(f"{label} docs {json.dumps(item['docs'])}")
        if attrs and item.get("attrs"):
            shown = json.dumps([attr(a) for a in item["attrs"]], sort_keys=True)
            out.add(f"{label} attrs {shown}")

    def path_of(item_id):
        p = paths.get(str(item_id))
        if p:
            return "::".join(p["path"])
        return index.get(str(item_id), {}).get("name")

    def field(item_id, label):
        f = index[str(item_id)]
        meta(f"{label} {f['name']}", f)
        return f"{f['name']} {f['visibility']} {show(f['inner']['struct_field'])}"

    def impls(here, impl_ids):
        for impl_id in impl_ids:
            imp = index[str(impl_id)]["inner"]["impl"]
            if imp.get("is_synthetic") or imp.get("blanket_impl"):
                continue
            head = f"impl {show(imp['trait']) if imp['trait'] else '<inherent>'} for {here} {show(imp['for'])}"
            out.add(head)
            meta(head, index[str(impl_id)])
            for member in imp["items"]:
                m = index[str(member)]
                out.add(f"{head} :: {m['name']} {show(m['inner'])}")
                meta(f"{head} :: {m['name']}", m)

    for item_id, item in index.items():
        if item.get("crate_id") != 0:
            continue
        kind, body = next(iter(item["inner"].items()))
        here = path_of(item_id) or item.get("name")
        if kind in ("struct", "enum", "constant", "trait", "function", "type_alias", "static"):
            meta(f"{kind} {here}", item)
        if kind == "struct":
            k = body["kind"]
            if "plain" in k:
                for f in k["plain"]["fields"]:
                    out.add(f"struct {here} field {field(f, f'struct {here} field')}")
                if k["plain"].get("has_stripped_fields"):
                    out.add(f"struct {here} has private fields")
            elif "tuple" in k:
                for n, f in enumerate(k["tuple"]):
                    shown = "private" if f is None else field(f, f"struct {here} .{n}")
                    out.add(f"struct {here} .{n} {shown}")
            else:
                out.add(f"struct {here} {show(k)}")
            impls(here, body["impls"])
        elif kind == "enum":
            for v in body["variants"]:
                variant = index[str(v)]
                label = f"enum {here} variant {variant['name']}"
                meta(label, variant)
                inner = variant["inner"]["variant"]
                disc = (inner.get("discriminant") or {}).get("value")
                vk = inner["kind"]
                if isinstance(vk, dict) and "struct" in vk:
                    fields = [field(f, f"{label} field") for f in vk["struct"]["fields"]]
                elif isinstance(vk, dict) and "tuple" in vk:
                    fields = [
                        field(f, f"{label} field") if f is not None else "private"
                        for f in vk["tuple"]
                    ]
                else:
                    fields = []
                out.add(f"enum {here} variant {variant['name']} = {disc} {fields}")
            impls(here, body["impls"])
        elif kind == "constant":
            out.add(f"const {here} {show(body['type'])} = {body['const'].get('value')}")
        elif kind == "trait":
            for member in body["items"]:
                m = index[str(member)]
                out.add(f"trait {here} :: {m['name']} {show(m['inner'])}")
                meta(f"trait {here} :: {m['name']}", m)
        elif kind in ("function", "type_alias", "static"):
            out.add(f"{kind} {here} {show(body)}")
    return sorted(out)


def main():
    parser = argparse.ArgumentParser(
        description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter
    )
    parser.add_argument("--docs", action="store_true")
    parser.add_argument("--attrs", action="store_true")
    parser.add_argument("json")
    args = parser.parse_args()
    with open(args.json) as f:
        doc = json.load(f)
    for line in summarize(doc, docs=args.docs, attrs=args.attrs):
        print(line)


main()
