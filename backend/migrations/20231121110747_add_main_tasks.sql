-- Add migration script here
INSERT INTO task_group (id, title, slug, legend) VALUES (2, "Основные автоматы", "fa", "Эти задачи демонстрируют основные принципы использования автоматов для определения регулярных языков");

INSERT INTO task (group_id, slug, title, legend, script, model_solution_json) VALUES (
2,
"even-letter-count",
"Четное количество букв",
"Создайте автомат, который принимает слова, где есть четное количество букв Q (в том числе ноль, и в том числе пустое слово). Слово также может содержать буквы W, которые нужно игнорировать.",
'fn gen_word(ok) {
    let length = rng.gen_range(0, 30);
    let word = "";
    let chars = ["Q", "W"];
    for i in 0..length {
      word += chars[rng.gen_range(0,1)];
    }
    if !(check_word(word) == ok) {
        word += "Q";
    }
    return word;
}

fn check_word(word) {
    let count = 0;
    for ch in word {
      if ch == "Q" {
        count += 1;
      }
    }
    (count % 2) == 0
}',
'{"nodes":[{"x":276,"y":210,"text":"","isAcceptState":true},{"x":459,"y":210,"text":"","isAcceptState":false}],"links":[{"type":"StartLink","node":0,"text":"","deltaX":-113,"deltaY":0},{"type":"Link","nodeA":0,"nodeB":1,"text":"Q","lineAngleAdjust":3.141592653589793,"parallelPart":0.6947368421052632,"perpendicularPart":-57.0},{"type":"Link","nodeA":1,"nodeB":0,"text":"Q","lineAngleAdjust":3.141592653589793,"parallelPart":0.6612021857923497,"perpendicularPart":-57.0},{"type":"SelfLink","node":0,"text":"W","anchorAngle":-2.0131705497716417},{"type":"SelfLink","node":1,"text":"W","anchorAngle":-0.5585993153435624}]}');

INSERT INTO task (group_id, slug, title, legend, script, model_solution_json) VALUES (
2,
"same-parity",
"Одинаковая четность",
"Создайте автомат, который принимает слова, в которых количество букв J и L имеет одинаковую четность: либо обоих букв четное количество, либо обоих нечетное. Пустое слово также следует принимать.",
'fn gen_word(ok) {
    let length = rng.gen_range(0, 40);
    let word = "";
    let chars = ["J", "L"];
    for i in 0..length {
      word += chars[rng.gen_range(0,1)];
    }
    if !(check_word(word) == ok) {
        word += "L";
    }
    return word;
}

fn check_word(word) {
    let countj = 0;
    let countl = 0;
    for ch in word {
      if ch == "J" {
        countj += 1;
      }

      if ch == "L" {
        countl += 1;
      }
    }
    (countj % 2) == (countl % 2)
}',
'{"nodes":[{"x":194,"y":160,"text":"","isAcceptState":true},{"x":535,"y":160,"text":"","isAcceptState":false},{"x":194,"y":406,"text":"","isAcceptState":false},{"x":535,"y":406,"text":"","isAcceptState":true}],"links":[{"type":"StartLink","node":0,"text":"","deltaX":-113,"deltaY":0},{"type":"Link","nodeA":0,"nodeB":1,"text":"J","lineAngleAdjust":3.141592653589793,"parallelPart":0.7478005865102639,"perpendicularPart":-53.0},{"type":"Link","nodeA":1,"nodeB":0,"text":"J","lineAngleAdjust":3.141592653589793,"parallelPart":0.6275659824046921,"perpendicularPart":-48.0},{"type":"Link","nodeA":0,"nodeB":2,"text":"L","lineAngleAdjust":0.0,"parallelPart":0.7764227642276422,"perpendicularPart":24.0},{"type":"Link","nodeA":2,"nodeB":0,"text":"L","lineAngleAdjust":0.0,"parallelPart":0.6138211382113821,"perpendicularPart":36.0},{"type":"Link","nodeA":2,"nodeB":3,"text":"J","lineAngleAdjust":3.141592653589793,"parallelPart":0.8504398826979472,"perpendicularPart":-26.0},{"type":"Link","nodeA":3,"nodeB":2,"text":"J","lineAngleAdjust":3.141592653589793,"parallelPart":0.7536656891495601,"perpendicularPart":-31.0},{"type":"Link","nodeA":3,"nodeB":1,"text":"L","lineAngleAdjust":0.0,"parallelPart":0.7804878048780488,"perpendicularPart":30.0},{"type":"Link","nodeA":1,"nodeB":3,"text":"L","lineAngleAdjust":0.0,"parallelPart":0.8252032520325203,"perpendicularPart":19.0}]}');

INSERT INTO task (group_id, slug, title, legend, script, model_solution_json) VALUES (
2,
"exactly-two-letters",
"Ровно две буквы",
"Создайте автомат, который принимает слова, состоящие из букв A, B, C и D, в которых есть ровно две буквы D; слова, где больше или меньше букв D следует не принимать.",
'
fn gen_word(ok) {
    let word = "";
    if ok == false {
      word = "DD";
    }
    let chars = ["A", "B", "C", "D"];
    while !(check_word(word) == ok) {
        let length = rng.gen_range(0, 40);
        word = "";
        for i in 0..length {
          word += chars[rng.gen_range(0,3)];
        }
    }
    return word;
}

fn check_word(word) {
    let countd = 0;
    for ch in word {
      if ch == "D" {
        countd += 1;
      }
    }
    (countd == 2)
}',
'{"nodes":[{"x":218,"y":128,"text":"","isAcceptState":false},{"x":377,"y":128,"text":"","isAcceptState":false},{"x":534,"y":128,"text":"","isAcceptState":true},{"x":678,"y":128,"text":"","isAcceptState":false}],"links":[{"type":"StartLink","node":0,"text":"","deltaX":-87,"deltaY":0},{"type":"Link","nodeA":0,"nodeB":1,"text":"D","lineAngleAdjust":3.141592653589793,"parallelPart":0.717948717948718,"perpendicularPart":-55.0},{"type":"Link","nodeA":1,"nodeB":2,"text":"D","lineAngleAdjust":3.141592653589793,"parallelPart":0.7515923566878981,"perpendicularPart":-61.0},{"type":"Link","nodeA":2,"nodeB":3,"text":"D","lineAngleAdjust":3.141592653589793,"parallelPart":0.7708333333333334,"perpendicularPart":-54.0},{"type":"SelfLink","node":0,"text":"A","anchorAngle":2.1112158270654806},{"type":"SelfLink","node":0,"text":"B","anchorAngle":1.5707963267948966},{"type":"SelfLink","node":0,"text":"C","anchorAngle":0.851966327173272},{"type":"SelfLink","node":1,"text":"A","anchorAngle":2.2455372690184494},{"type":"SelfLink","node":1,"text":"B","anchorAngle":1.5707963267948966},{"type":"SelfLink","node":1,"text":"C","anchorAngle":0.8960553845713439},{"type":"SelfLink","node":2,"text":"A","anchorAngle":2.2390857456254807},{"type":"SelfLink","node":2,"text":"B","anchorAngle":1.5707963267948966},{"type":"SelfLink","node":2,"text":"C","anchorAngle":0.8124186125847132}]}');

INSERT INTO task (group_id, slug, title, legend, script, model_solution_json) VALUES (
2,
"no-three-ones",
"Нет трех единиц",
"Создайте автомат, который принимает только те строки из нулей и единиц, которые не содержат трех единиц подряд: те, которые не содержат '111'. ",
'
fn gen_word(ok) {
    let seq_one_count = 0;
    let did_fail = false;
    let length = rng.gen_range(0, 40);
    let word = "";
    for i in 0..length {
      let new_digit = rng.gen_range(0,1);
      if new_digit == 1 {
        if seq_one_count == 2 && ok {
          new_digit = 0;
        }
      }
      if new_digit == 1 {
        seq_one_count += 1;
        if seq_one_count == 3 { did_fail = true;  }
      } else { seq_one_count = 0; }
      word = word + new_digit;
    }
    if !ok && !did_fail {
      word = word + "111";
    }
    return word;
}

fn check_word(word) {
  !word.contains("111")
}',
'{"nodes":[{"x":362,"y":318,"text":"","isAcceptState":true},{"x":362,"y":459,"text":"","isAcceptState":true},{"x":362,"y":188,"text":"","isAcceptState":true}],"links":[{"type":"StartLink","node":1,"text":"","deltaX":-155,"deltaY":19},{"type":"SelfLink","node":1,"text":"0","anchorAngle":0.6517921949413541},{"type":"Link","nodeA":0,"nodeB":1,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.5815602836879432,"perpendicularPart":60.0},{"type":"Link","nodeA":2,"nodeB":1,"text":"0","lineAngleAdjust":3.141592653589793,"parallelPart":0.5018450184501845,"perpendicularPart":-87.0},{"type":"Link","nodeA":1,"nodeB":0,"text":"1","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":0,"nodeB":2,"text":"1","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0}]}');
