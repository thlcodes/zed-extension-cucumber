[
  (background_kw)
  (examples_kw)
  (feature_kw)
  (rule_kw)
  (scenario_kw)
  (scenario_outline_kw)
] @keyword

(given_group
  [
    (given_step
      (given_line
        (given_kw) @keyword.import))
    (asterisk_step
      (asterisk_line
        "* " @keyword.import))
    (and_step
      (and_line
        (and_kw) @keyword.import))
    (but_step
      (but_line
        (but_kw) @keyword.import))
  ])

(when_group
  [
    (when_step
      (when_line
        (when_kw) @function))
    (asterisk_step
      (asterisk_line
        "* " @function))
    (and_step
      (and_line
        (and_kw) @function))
    (but_step
      (but_line
        (but_kw) @function))
  ])

(then_group
  [
    (then_step
      (then_line
        (then_kw) @type))
    (asterisk_step
      (asterisk_line
        "* " @type))
    (and_step
      (and_line
        (and_kw) @type))
    (but_step
      (but_line
        (but_kw) @type))
  ])

(step_param) @variable.parameter

(step_quoted_string) @type

(step_context) @string

(tag) @type

(table_head_row
  (table_col
    (table_cell) @markup.heading))

(data_table
  (table_row
    (table_col
      (table_cell) @string)))

(description) @comment.doc

":" @punctuation.delimiter

"|" @punctuation.special

(doc_string) @string.documentation
(doc_string_content) @string
(media_type) @type

"language" @property

(language_name) @string
(invalid_language_name) @comment.error

[
  (comment)
  (language)
] @comment
