// Уникальные особенности Rust в сравнении с классическими языками (C/C++/Java/Go)

// 1. Система владения и заимствования
fn ownership_example() {
  let s1 = String::from("hello");
  let s2 = s1; // Rust уникален: s1 перемещается в s2, s1 больше недоступна
  // println!("{}", s1); // Ошибка компиляции: значение s1 использовано после перемещения
  println!("{}", s2); // OK

  let s3 = String::from("world");
  let len = calculate_length(&s3); // Заимствование без передачи владения
  println!("Длина '{}' - {}", s3, len); // s3 все еще доступна
}

fn calculate_length(s: &String) -> usize { s.len() }

// 2. Времена жизни
// Уникальная концепция Rust: явное указание времен жизни для предотвращения висячих ссылок
fn lifetime_example<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() { x } else { y }
}

// 3. Отсутствие null, использование Option
fn option_example(x: Option<i32>) -> Option<i32> {
  match x {
      Some(i) => Some(i + 1), // Rust заставляет явно обрабатывать наличие значения
      None => None,           // и его отсутствие без использования null
  }
}

// 4. Результат с обработкой ошибок (вместо исключений)
fn result_example(x: i32, y: i32) -> Result<i32, String> {
  if y == 0 {
      Err(String::from("Деление на ноль")) // Вместо исключений: явное возвращение ошибки
  } else {
      Ok(x / y)
  }
}

// 5. Сопоставление с образцом
enum Message { Quit, Move { x: i32, y: i32 }, Write(String) }

fn pattern_matching(msg: Message) {
  match msg {
      // Rust требует исчерпывающей проверки всех вариантов
      Message::Quit => println!("Выход"),
      Message::Move { x, y } => println!("Перемещение на {}, {}", x, y),
      Message::Write(text) => println!("Текст сообщения: {}", text),
  }
}

// 6. Трейты (интерфейсы с реализацией по умолчанию)
trait Animal {
  fn make_sound(&self) -> String;
  // В отличие от интерфейсов в других языках: метод с реализацией по умолчанию
  fn description(&self) -> String {
      format!("Животное, которое издает звук: {}", self.make_sound())
  }
}

struct Dog;
impl Animal for Dog {
  fn make_sound(&self) -> String { String::from("Гав") }
  // description() использует реализацию по умолчанию
}

// 7. Безопасное использование многопоточности
use std::thread;
use std::sync::mpsc;

fn thread_example() {
  let (tx, rx) = mpsc::channel();

  thread::spawn(move || {
      // Rust гарантирует: безопасная передача данных между потоками
      tx.send("Привет из потока").unwrap();
  });

  println!("Получено: {}", rx.recv().unwrap());
}

// 8. Макросы (более мощные, чем в C/C++)
macro_rules! create_function {
  ($func_name:ident) => {
      // Возможность Rust: макросы могут создавать функции
      fn $func_name() {
          println!("Вы вызвали функцию {:?}()", stringify!($func_name));
      }
  };
}

create_function!(foo);

// 9. Отсутствие неявных преобразований
fn no_implicit_conversion() {
  let x: i32 = 5;
  // let y: i64 = x; // Ошибка: требуется явное преобразование
  let y: i64 = x as i64; // Особенность Rust: необходимо явное приведение типов
  println!("y = {}", y);
}

// 10. Изменяемость не наследуется
fn mutability_example() {
  let mut x = 5;
  let y = &mut x; // Важно в Rust: явное указание изменяемости ссылки
  *y += 1;        // Изменение через изменяемую ссылку
  println!("x = {}", x);
  // let z = &x;  // Ошибка: нельзя одновременно иметь изменяемую и неизменяемую ссылку
}

fn main() {
  ownership_example();

  let s1 = String::from("short");
  let s2 = String::from("longer");
  println!("Длиннее: {}", lifetime_example(&s1, &s2));

  println!("Option: {:?}", option_example(Some(41)));

  match result_example(10, 2) {
      Ok(result) => println!("Результат: {}", result),
      Err(e) => println!("Ошибка: {}", e),
  }

  pattern_matching(Message::Move { x: 3, y: 4 });

  let dog = Dog;
  println!("{}", dog.description());

  thread_example();

  foo();

  no_implicit_conversion();

  mutability_example();
}
