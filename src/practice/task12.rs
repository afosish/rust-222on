#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy)]
struct Rectangle {
    a: Point,
    b: Point,
}

fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle {
            a: Point { x: 2, y: 9 },
            b: Point { x: 5, y: 3 },
        },
        Rectangle {
            a: Point { x: 1, y: 8 },
            b: Point { x: 11, y: 6 },
        },
        Rectangle {
            a: Point { x: 9, y: 10 },
            b: Point { x: 13, y: 2 },
        },
    ]
}

fn area_occupied(rectangles: &Vec<Rectangle>) -> i32 {
    // Збираємо усі події відкриття і закриття прямокутників по осі X
    let mut events = Vec::new();
    for rect in rectangles {
        events.push((rect.a.x, rect.a.y, rect.b.y, 1)); // відкриття прямокутника
        events.push((rect.b.x, rect.a.y, rect.b.y, -1)); // закриття прямокутника
    }
    // Сортуємо події за X-координатою
    events.sort();

    let mut prev_x = events[0].0;
    let mut total_area = 0;

    // Відкриті інтервали по осі Y
    let mut active_intervals = Vec::new();

    // Проходимо по подіях
    for event in events {
        let (x, y1, y2, delta) = event;

        // Оновлюємо площу на основі активних інтервалів по осі Y
        let covered_y = calculate_covered_y(&active_intervals);
        total_area += covered_y * (x - prev_x);

        // Оновлюємо попереднє значення X
        prev_x = x;

        // Оновлюємо список активних інтервалів по Y
        if delta == 1 {
            active_intervals.push((y1, y2));
        } else {
            if let Some(pos) = active_intervals.iter().position(|&interval| interval == (y1, y2)) {
                active_intervals.remove(pos);
            }
        }
    }

    total_area
}

// Функція для підрахунку загальної зайнятої довжини вздовж осі Y
fn calculate_covered_y(intervals: &Vec<(i32, i32)>) -> i32 {
    let mut sorted_intervals = intervals.clone();
    sorted_intervals.sort();

    let mut total_length = 0;
    let mut current_start = sorted_intervals[0].0;
    let mut current_end = sorted_intervals[0].1;

    for &(start, end) in &sorted_intervals[1..] {
        if start > current_end {
            total_length += current_end - current_start;
            current_start = start;
            current_end = end;
        } else {
            current_end = current_end.max(end);
        }
    }

    total_length += current_end - current_start;
    total_length
}

fn area_occupied_test() {
    let data = test_data();
    let occupied = area_occupied(&data);
    assert_eq!(occupied, 60);
    println!("Тест пройдено. Фактично зайнята площа: {}", occupied);
}
#[test]
fn main() {
    area_occupied_test();
}
