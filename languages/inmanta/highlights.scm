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
(dict_kw) @keyword.type

; ── Operators ─────────────────────────────────────────────────────────────────
(cmp_op) @operator
(rel) @operator
(peq) @operator
(double_star) @operator
(plus_op) @operator
(minus_op) @operator
(division_op) @operator
(mod_op) @operator

; ── Punctuation ───────────────────────────────────────────────────────────────
"(" @punctuation.bracket
")" @punctuation.bracket
"[" @punctuation.bracket
"]" @punctuation.bracket
"{" @punctuation.bracket
"}" @punctuation.bracket
"," @punctuation.delimiter
"." @punctuation.delimiter
(sep) @punctuation.delimiter
":" @punctuation.delimiter
