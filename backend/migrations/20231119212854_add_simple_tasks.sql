-- Add migration script here
INSERT INTO task_group (id, title, slug, legend) VALUES (1, "Simple tasks", "simple", "These tasks are a simple showcase of the features of the service");

INSERT INTO task (id, group_id, slug, title, legend, script, model_solution_json) VALUES (1,
1,
"only-ones",
"Only 1s",
"Create a FSM that accepts any word consisting of any number of 1s. Any other word, including the empty word, must be rejected.",
'fn gen_word(r, ok) {
    let length = call(r, 0, 50);
    let word = "";
    if ok {
        for i in 0..length {
            word += "1";
        }
    } else {
        for i in 0..length {
            word += call(r, 0, 1);
        }
    }
    
    if !(check_word(word) == ok) {
        word += "0";
    }
    return word;
}

fn check_word(w) {
    if word.is_empty() {
        return false;
    }
    while !word.is_empty() {
        if w.pop() != "1" {
            return false;
        }
    }
    return true;
}',
'{"nodes":[{"x":291,"y":334,"text":"","isAcceptState":true}],"links":[{"type":"StartLink","node":0,"text":"","deltaX":-90,"deltaY":0},{"type":"SelfLink","node":0,"text":"1","anchorAngle":-1.5707963267948966}]}')