-- Add migration script here
INSERT INTO task_group (id, title, slug, legend) VALUES (1, "Simple tasks", "simple", "These tasks are a simple showcase of the features of the service");

INSERT INTO task (id, group_id, slug, title, legend, script, model_solution_json) VALUES (1,
1,
"only-ones",
"Only 1s",
"Create a FSM that accepts strings consisting of any number of 1s, and rejects anything else.",
'',
'{"nodes":[{"x":291,"y":334,"text":"","isAcceptState":true}],"links":[{"type":"StartLink","node":0,"text":"","deltaX":-90,"deltaY":0},{"type":"SelfLink","node":0,"text":"1","anchorAngle":-1.5707963267948966}]}')