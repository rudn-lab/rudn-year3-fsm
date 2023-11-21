-- Add migration script here
INSERT INTO task_group (id, title, slug, legend) VALUES (1, "Простые задачи", "simple", "Эти задачи служат для проверки возможностей тестирующей системы");

INSERT INTO task (group_id, slug, title, legend, script, model_solution_json) VALUES (
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
'{"nodes":[{"x":291,"y":334,"text":"","isAcceptState":true}],"links":[{"type":"StartLink","node":0,"text":"","deltaX":-90,"deltaY":0},{"type":"SelfLink","node":0,"text":"1","anchorAngle":-1.5707963267948966}]}');

INSERT INTO task (group_id, slug, title, legend, script, model_solution_json) VALUES (
1,
"fixed-lang",
"Фиксированный язык",
"Создайте автомат, который принимает только эти слова (без кавычек): пустое слово, '01', '011', '1', '11', '111'.",
'
fn get_ok_words() {
    ["", "01", "011", "1", "11", "111"]
}

fn gen_word(ok) {
    let ok_words = get_ok_words();
    if ok {
        return ok_words[rng.gen_range(0, ok_words.len-1)];
    } else {
        let word = "";
        while check_word(word) {
            word = "";
            let len = rng.gen_range(1, 10);
            for i in 0..len {
                word += rng.gen_range(0, 1);
            }
        }
        return word;
    }
}

fn check_word(word) {
    let ok_words = get_ok_words();
    ok_words.contains(word)
}
',
'{"nodes":[{"x":138,"y":351,"text":"","isAcceptState":false},{"x":258,"y":236,"text":"","isAcceptState":false},{"x":427,"y":236,"text":"","isAcceptState":true},{"x":590,"y":236,"text":"","isAcceptState":true},{"x":288,"y":436,"text":"","isAcceptState":true},{"x":151,"y":492,"text":"","isAcceptState":true}],"links":[{"type":"StartLink","node":0,"text":"","deltaX":-78,"deltaY":0},{"type":"Link","nodeA":0,"nodeB":1,"text":"0","lineAngleAdjust":3.141592653589793,"parallelPart":0.32555448408871746,"perpendicularPart":0.0},{"type":"Link","nodeA":1,"nodeB":2,"text":"1","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":2,"nodeB":3,"text":"1","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":0,"nodeB":4,"text":"1","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":4,"nodeB":2,"text":"1","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":0,"nodeB":5,"text":"","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0}]}');

INSERT INTO task (group_id, slug, title, legend, script, model_solution_json) VALUES (
1,
"divisible-by-10",
"Делится на 10",
"Создайте автомат, который принимает только слова, которые являются десятичными числами, кратными 10",
'

fn gen_word(ok) {
    let letters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    let num = rng.gen_range(0, 1000) * 10;
    if !ok {
        num = num + rng.gen_range(1, 9);

        if rng.gen_range(0, 20) == 0 {
            let word = "10";
            while check_word(word) {
                word = "";
                let len = rng.gen_range(1, 15);
                for i in 0..len {
                    if rng.gen_range(0, 4) == 0 {
                        word += letters[rng.gen_range(0, letters.len-1)];
                    } else {
                        word += rng.gen_range(0,9);
                    }
                }
            }
            return word;
        }
    }
    return "" + num;
}

fn check_word(word) {
    if word.len == 0 {return false;}
    let digits = "0123456789";
    if word[-1] == "0" {
        for ch in word {
            if !digits.contains(ch) {
                return false;
            }
        }
        return true;
    } else {
        false
    }
}
',
'');