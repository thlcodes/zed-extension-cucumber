; Run entire feature
(feature
  (feature_header
    (feature_line
      (feature_kw) @run
      (context) @feature_name))
  (#set! tag cucumber-feature))

; Run individual scenario
(scenario_definition
  (scenario
    (scenario_line
      (scenario_kw) @run
      (context) @scenario_name))
  (#set! tag cucumber-scenario))

; Run scenario outline
(scenario_definition
  (scenario
    (scenario_outline_line
      (scenario_outline_kw) @run
      (context) @scenario_name))
  (#set! tag cucumber-scenario))
