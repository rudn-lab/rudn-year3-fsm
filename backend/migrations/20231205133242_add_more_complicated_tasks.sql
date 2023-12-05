-- Add migration script here
INSERT INTO task_group (id, title, slug, legend) VALUES (3, "Недетерминированные автоматы", "nfa", "Эти задачи задают формальные языки, которые легче всего описать с помощью НКА");

INSERT INTO task (group_id, slug, title, legend, script, model_solution_json) VALUES (
3,
"ends-with",
"Оканчивается на",
"Создайте автомат, который принимает слова из нулей и единиц, где последние буквы слова равны '11010'.",
'fn gen_word(ok) {
    let length = rng.gen_range(0, 30);
    let word = "";
    for i in 0..length {
      word += rng.gen_range(0,1);
    }
    if ok {
        if !check_word(word){
            word = word + "11010";
        }
    } else {
        if check_word(word){
            word = word + "0";
        }
    }
    return word;
}

fn check_word(word) {
    word.ends_with("11010")
}',
'');

INSERT INTO task (group_id, slug, title, legend, script, model_solution_json) VALUES (
3,
"branching-start",
"Одно или другое начало",
"Создайте автомат, который принимает слова из нулей и единиц, которые начинаются либо на '101', либо на '010', и после этого содержат хотя бы одну еще цифру (то есть минимальная длина принимаемого слова равна 4).",
'fn gen_word(ok) {
    let length = rng.gen_range(1, 15);
    let word = "";
    if ok {
        if rng.gen_range(0,1) == 0 {
            word = "010";
        } else {
            word = "101";
        }
        for i in 0..length {
            word += rng.gen_range(0,1);
        }
        return word;
    } else {
        for i in 0..length {
            word += rng.gen_range(0,1);
        }
        while check_word(word) {
            word = rng.gen_range(0,1) + word;
        }
        return word;
    }
}

fn check_word(word) {
    (word.starts_with("101") || word.starts_with("010")) && word.len() >= 4
}',
'');

INSERT INTO task (group_id, slug, title, legend, script, model_solution_json) VALUES (
3,
"from-end-counter",
"Отсчитать от конца",
"Создайте автомат, который принимает слова из нулей и единиц, где третья цифра с конца равна 1. В том числе, следует принимать слова длиной 3, начинающиеся с 1.",
'fn gen_word(ok) {
    let length = rng.gen_range(3, 15);
    let word = "";
    for i in 0..length {
        word += rng.gen_range(0,1);
    }
    if ok {
        word[length - 3] = ''1'';
    } else {
        word[length - 3] = ''0'';
    }
    return word;
}

fn check_word(word) {
    if word.len() < 3 {return false;}
    word[word.len() - 3] == "1"
}',
'');

INSERT INTO task (group_id, slug, title, legend, script, model_solution_json) VALUES (
3,
"palindrome",
"Палиндром",
"Создайте автомат, который принимает слова из букв A, B и C, которые имеют длину 4 и являются палиндромами (то есть первая буква равна последней, а вторая равна третьей).",
'fn gen_word(ok) {
    let letters = ["A", "B", "C"];
    if ok {
        let word = letters[rng.gen_range(0,2)];
        word += letters[rng.gen_range(0,2)];
        word += word[1];
        word += word[0];
        return word;
    }
    else {
        let word = letters[rng.gen_range(0,2)];
        word += letters[rng.gen_range(0,2)];
        word += letters[rng.gen_range(0,2)];
        let last = letters[rng.gen_range(0,2)];
        while word[0] == last {
            last = letters[rng.gen_range(0,2)];
        }
        word += last;
        return word;
    }
}

fn check_word(word) {
    if word.len() != 4 {return false;}
    word[0] == word[3] && word[1] == word[2]
}',
'');


INSERT INTO task_group (id, title, slug, legend) VALUES (4, "Почти практически полезные задачи", "practical", "Формальные языки, которые нужно реализовать в этих задачах, напоминают часто встречающиеся требования в настоящем программировании");

INSERT INTO task (group_id, slug, title, legend, script, model_solution_json) VALUES (
4,
"ipv4",
"IPv4-адрес",
"Создайте автомат, который принимает слова, где есть четыре части. Каждая из этих частей может содержать одну, две или три решетки (#). Между частями находятся точки.",
'fn gen_word(ok) {
    let nums = ["#","##","###"];

    let a = nums[rng.gen_range(0,2)];
    let b = nums[rng.gen_range(0,2)];
    let c = nums[rng.gen_range(0,2)];
    let d = nums[rng.gen_range(0,2)];
    let e = nums[rng.gen_range(0,2)];

    let word = a+"."+b+"."+c+"."+d;
    if ok {
        return word;
    }
    else {
        while check_word(word) {
            if rng.gen_range(0, 20) == 0 {a = "";}
            if rng.gen_range(0, 20)==0 {
                a += "#";
            }
            if rng.gen_range(0, 20) == 0 {b = "";}
            if rng.gen_range(0, 20)==0 {
                b += "#";
            }
            if rng.gen_range(0, 20) == 0 {c = "";}
            if rng.gen_range(0, 20)==0 {
                c += "#";
            }
            if rng.gen_range(0, 20) == 0 {d = "";}
            if rng.gen_range(0, 20)==0 {
                d += "#";
            }
            if rng.gen_range(0, 20) == 0 {e = "";}
            if rng.gen_range(0, 20)==0 {
                e += "#";
            }


            if rng.gen_range(0, 30) == 0 {
                if rng.gen_range(0, 1) == 0 {
                    word = a + "." + b + "." + c;
                } else {
                    word = a + "." + b + "." + c + "." + d + "." + e;
                }
            }
            else {
                word = a + "." + b + "." + c + "." + d;

            }
        }
    }
    return word;
}

fn check_word(word) {
    let nums = ["#","##","###"];

    let parts = word.split(".");
    if parts.len() != 4 {return false;}
    for part in parts {
        if nums.contains(part) {continue;}
        return false;
    }

    true
}',
'');

INSERT INTO task (group_id, slug, title, legend, script, model_solution_json) VALUES (
4,
"phone-number",
"Номер телефона",
"Создайте автомат, который принимает слова, где в начале может быть или не быть '+7', а затем идут 10 решеток (#). Они разделены на группы: 3, 3, 2, 2. Эти группы могут быть или не быть разделены знаками минус каким угодно способом, за исключением, что первая группа также может быть в скобках, и если это так, то рядом с ней не может быть минусов; также, строка не может начинаться с минуса. (Щелкните на кнопку 'Примеры', чтобы посмотреть примеры таких строк!)",
'fn gen_word(ok) {
    if !ok {
        let word = gen_bad(rng);
        while check_word(word) {
            rng.gen_range(0,1);
            word = gen_bad(rng);
        }
        return word;
    }
    let word = "";

    if rng.gen_range(0,5) == 0 {
        word += "+7";
    }

    let did_bracket = rng.gen_range(0,3) == 0;
    if did_bracket {
        word += "(###)";
    } else {
        if rng.gen_range(0,1) == 0 && !(word.len() == 0) {
            word += "-";
        }
        word += "###";
        if rng.gen_range(0,1) == 0 {
            word += "-";
        }
    }

    word += "###";
    if rng.gen_range(0,1) == 0 {
        word += "-";
    }
    word += "##";
    if rng.gen_range(0,1) == 0 {
        word += "-";
    }
    word += "##";
    return word;
}

fn gen_bad(rng) {
    let word = "";
    // plus part
    if rng.gen_range(0,5)==0 {
        word += "+";
        if rng.gen_range(0,10) == 0 {
            word += rng.gen_range(0, 6);
        }
    }
    // bracketable part
    if rng.gen_range(0,3)==0 {
        // do bracket
        if rng.gen_range(0,4) == 0 {
            if rng.gen_range(0,1) == 0 {
                word += "(##)";
            } else {
                word += "(####)";
            }
        }
    } else {
        if rng.gen_range(0,5) == 0 {
            word += "-";
        }
        if rng.gen_range(0,8) == 0 {
            if rng.gen_range(0,1) == 0 {
                word += "##";
            } else {
                word += "####";
            }
        } else {word += "###";}
        if rng.gen_range(0,5) == 0 {
            word += "-";
        }

    }

    let remainder = "###-##-##";
    for start in 0..remainder.len() {
        let s = remainder[start];
        let other_idx = rng.gen_range(start, remainder.len() - 1);
        remainder[start] = remainder[other_idx];
        remainder[other_idx] = s;
    }

    if rng.gen_range(0, 20) == 0 {
        remainder += "#"
    }

    word += remainder;
    return word;
}



fn check_word(word) {
    try {
        if word.pop(2) != "##" {return false;}
        if word.ends_with("-") {word.pop();}
        if word.pop(2) != "##" {return false;}
        if word.ends_with("-") {word.pop();}
        if word.pop(3) != "###" {return false;}
        if word.ends_with(")") {
            // bracketed part
            word.pop();
            if word.pop(4) != "(###" {return false;}
        } else {
            // unbracketed part
            if word.ends_with("-") {word.pop();}
            if word.pop(3) != "###" {return false;}
            if word.ends_with("-") {word.pop();}
        }
        if word.is_empty() || word == "+7" {
            return true;
        } else {
            return false;
        }
    } catch {
        return false;
    }
}',
'');

INSERT INTO task (group_id, slug, title, legend, script, model_solution_json) VALUES (
4,
"hashtag",
"Хештег",
"Создайте автомат, который принимает слова, которые начинаются (#), а затем содержат одну и больше групп большых или маленьких букв A, которые могут быть разделены минусами и нижними подчеркиваниями; минус и нижнее подчеркивание не могут быть первой буквой после решетки или последней буквой, и не могут стоять рядом друг с другом.",
'fn gen_word(ok) {
    let letters = ["a", "A", "-", "_"];
    let word = "";
    if ok {
        word = "#";
        word += letters[rng.gen_range(0,1)];
        for i in 0..rng.gen_range(1,6) {
            for j in 0..rng.gen_range(1,5) {
                word += letters[rng.gen_range(0,1)];
            }
            if rng.gen_range(0,5) == 0 {
                word += letters[rng.gen_range(2,3)];
            }
        }
        if word.ends_with("_") || word.ends_with("-") {
            for j in 0..rng.gen_range(0,5) {
                word += letters[rng.gen_range(0,1)];
            }
        }
    } else {
        if rng.gen_range(0, 10) != 0 {
            word += "#";
        }
        for i in 0..rng.gen_range(0, 15) {
            word += letters[rng.gen_range(0,3)];
            if rng.gen_range(0, 20) == 0 {
                word += "#";
            }
        }
        if check_word(word) {
            word += letters[rng.gen_range(2,3)];
        }
    }
    return word;
}


fn check_word(word) {
    if !word.starts_with("#") {return false;}
    word = word.sub_string(1..word.len());
    if word.starts_with("-") || word.starts_with("_") || word.ends_with("-") || word.ends_with("_") {return false;}
    while word.contains("-") {word.remove("-");}
    while word.contains("_") {word.remove("_");}
    while word.contains("a") {word.remove("a");}
    while word.contains("A") {word.remove("A");}
    word.is_empty()
}',
'');