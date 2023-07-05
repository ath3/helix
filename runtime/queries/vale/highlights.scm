(type_identifier) @type
((field_identifier) @variable.other.member)

(parameter name: (identifier) @variable.parameter)

(scoped_identifier path: (identifier) @namespace)

(call_expr function: (field_expr  field: (field_identifier) @function))

(
 [
  (func_header name: (identifier) @function)
  (generic_function function: (identifier) @function)
  (call_expr function: (identifier) @function)
 ]
 (#match? @function "^[a-z]+([A-Za-z0-9_]+)*$")
)

(
 [
  (call_expr function: (identifier))
  (generic_function function: (identifier))
  (func_header name: (identifier))
 ] @constructor
 (#match? @constructor "^[A-Z]+([a-z]+[A-Z0-9_]*)*$")
)

(rune) @type

((identifier) @type (#match? @type "^[A-Z]"))

(struct_definition (type_identifier)) @struct

(attribute (attribute_name) @function.macro .)

(bool_lit) @constant.builtin.boolean
(int_lit) @constant.numeric.integer
(float_lit) @constant.numeric.float
(string_literal) @string
(escape_sequence) @string.escape

(variable_definition (pattern (identifier) @variable))

(line_comment) @comment
(region_parameter name: (identifier) @variable.other)
(region_specifier name: (identifier) @variable.other)

[
 "("
 ")"
 "["
 "]"
 "<"
 ">"
 "{"
 "}"
] @punctuation.bracket

[
 "."
 ","
 ";"
] @punctuation.delimiter

[
 "+"
 "-"
 "*"
 "/"
 "<"
 "<="
 ">"
 ">="
 "=="
 "==="
 "<=>"
 "!="
 "!"
 "="
 "=>"
 ; "mod"
 "xor"
 "rshift"
 "lshift"
] @operator

[
 "&"
 "&&"
 "^"
] @operator

"func" @keyword.function
"return" @keyword.return

[
 "else"
 "if"
] @keyword.conditional

[
 "while"
 "foreach"
 "in"
] @keyword.repeat

[
 "and"
 "or"
 "not"
] @keyword.control.conditional

[
 "imm"
 "mut"
 "set"
 "ro"
] @keyword.storage.modifier

"import" @keyword.control.import

[
 "extern"
 "exported"
 "pure"
 "struct"
] @keyword.storage.type

"'" @punctuation.special
(static_array_type ["#"] @punctuation.special)
(attribute ["#" "!"] @punctuation.special)
