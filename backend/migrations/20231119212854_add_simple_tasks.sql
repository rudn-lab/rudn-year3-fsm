-- Add migration script here
INSERT INTO task_group (id, title, slug, legend) VALUES (1, "Простые задачи", "simple", "Эти задачи служат для проверки возможностей тестирующей системы");

INSERT INTO task (id, group_id, slug, title, legend, script, model_solution_json) VALUES (1,
1,
"only-ones",
"Только единицы",
"Создайте автомат, который принимает слова, состоящие из любого количества единиц. Любое другое слово, включая пустое слово, следует не принимать.",
'fn gen_word(ok) {
    if rng.gen_range(0, 10) < 1 == 0 {
        return "";
    }
    let length = rng.gen_range(1, 25);
    let word = "";
    if ok {
        for i in 0..length {
            word += "1";
        }
    } else {
        for i in 0..length {
            word += rng.gen_range(0, 1);
        }
    }
    
    if !(check_word(word) == ok) {
        word += "0";
    }
    return word;
}

fn check_word(word) {
    if word.is_empty() {
        return false;
    }
    while !word.is_empty() {
        if word.pop() != "1" {
            return false;
        }
    }
    return true;
}',
'{"nodes":[{"x":291,"y":334,"text":"","isAcceptState":true}],"links":[{"type":"StartLink","node":0,"text":"","deltaX":-90,"deltaY":0},{"type":"SelfLink","node":0,"text":"1","anchorAngle":-1.5707963267948966}]}')