(arrow_function
	body: (_) @context.end
) @context

(function
  body: (_) @context.end
) @context

(function_declaration
	body: (_) @context.end
) @context

(
  (expression_statement
  	(call_expression
      	function: (_)))
  (statement_block) @context.end
) @context

[
	(call_expression)
	(class_declaration)
  (else_clause)
  (for_statement)
  (lexical_declaration)
  (lexical_declaration)
  (method_definition)
  (object)
  (pair)
  (while_statement)
] @context

