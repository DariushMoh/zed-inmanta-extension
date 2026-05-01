(attr_type_builtin) @type.builtin

; ── Comments ──────────────────────────────────────────────────────────────────
(comment) @comment
((comment) @comment.todo (#match? @comment.todo "TODO"))

; ── Strings ───────────────────────────────────────────────────────────────────
(string) @string
(mls) @string
(fstring) @string
(rstring) @string
(regex) @string.regexp

; ── Numbers ───────────────────────────────────────────────────────────────────
(integer) @number
(float) @number.float

; ── Constants ─────────────────────────────────────────────────────────────────
(null_kw) @constant.builtin
(true_kw) @boolean
(false_kw) @boolean
(undef_kw) @constant.builtin

; ── Types ─────────────────────────────────────────────────────────────────────
(cid) @type
(id) @variable

; ── Keywords ──────────────────────────────────────────────────────────────────
(entity_kw) @keyword
(end_kw) @keyword
(extends_kw) @keyword
(implementation_kw) @keyword
(implement_kw) @keyword
(using_kw) @keyword
(parents_kw) @keyword
(typedef_kw) @keyword.type
(as_kw) @keyword
(index_kw) @keyword
(import_kw) @keyword.import
(for_kw) @keyword.repeat
(if_kw) @keyword.conditional
(elif_kw) @keyword.conditional
(else_kw) @keyword.conditional
(when_kw) @keyword.conditional
(in_kw) @keyword.operator
(not_kw) @keyword.operator
(or_kw) @keyword.operator
(and_kw) @keyword.operator
(matching_kw) @keyword.operator
(is_kw) @keyword.operator
(defined_kw) @keyword.operator

; ── Operators ─────────────────────────────────────────────────────────────────
(cmp_op) @operator
(rel) @operator
(peq) @operator
(double_star) @operator
(plus_op) @operator
(minus_op) @operator
(division_op) @operator
(mod_op) @operator
(colon) @keyword.operator
(arity_colon) @operator
(ternary_then) @operator
(ternary_else) @operator

; ── Punctuation ───────────────────────────────────────────────────────────────
(call_open) @keyword.operator
(call_close) @keyword.operator
(dict_open) @keyword.operator
(dict_close) @keyword.operator
(lookup_open) @keyword.operator
(lookup_close) @keyword.operator
(arity_open) @keyword.operator
(arity_close) @keyword.operator
(list_open) @keyword.operator
(list_close) @keyword.operator
"," @punctuation.delimiter
"." @punctuation.delimiter
(sep) @punctuation.delimiter
