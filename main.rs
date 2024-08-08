// Полный краткий учебник по Rust с объяснениями

// 1. Основные типы данных и переменные
fn basic_types() {
  let x: i32 = 5;          // i32: 32-битное целое число со знаком
  let y: f64 = 3.14;       // f64: 64-битное число с плавающей точкой
  let b: bool = true;      // bool: логический тип (true или false)
  let c: char = 'a';       // char: символ в Unicode
  let s: &str = "Hello";   // &str: строковый срез (ссылка на UTF-8 строку)

  println!("x: {}, y: {}, b: {}, c: {}, s: {}", x, y, b, c, s);

  let mut mutable = 42;    // mut: позволяет изменять значение переменной
  mutable += 1;            // Используем mutable, чтобы избежать предупреждения
  println!("mutable: {}", mutable);
}

// 2. Управление потоком выполнения
fn control_flow(n: i32) {
  // if-else: условное выполнение
  if n > 0 {
      println!("Positive");
  } else if n < 0 {
      println!("Negative");
  } else {
      println!("Zero");
  }

  // for: цикл по диапазону
  for i in 0..3 {  // 0..3 создает диапазон от 0 до 2 включительно
      println!("Iteration {}", i);
  }

  // while: цикл с условием
  let mut counter = 0;
  while counter < 3 {
      println!("Counter: {}", counter);
      counter += 1;
  }
}

// 3. Владение (Ownership) и заимствование (Borrowing)
fn ownership_and_borrowing() {
  // Владение: каждое значение имеет переменную, которая называется его владельцем
  let s1 = String::from("hello");
  let s2 = s1; // s1 перемещается в s2, s1 больше недоступна
  // println!("{}", s1); // Это вызовет ошибку компиляции
  println!("s2: {}", s2);

  // Заимствование: создание ссылки без передачи владения
  let s3 = String::from("world");
  print_string(&s3); // & создает неизменяемую ссылку
  println!("s3: {}", s3); // s3 все еще доступна

  // Изменяемое заимствование
  let mut s4 = String::from("hello");
  change(&mut s4); // &mut создает изменяемую ссылку
  println!("s4: {}", s4);
}

fn print_string(s: &String) { // &String: неизменяемая ссылка на String
  println!("{}", s);
}

fn change(s: &mut String) { // &mut String: изменяемая ссылка на String
  s.push_str(", world");
}

// 4. Времена жизни (Lifetimes)
// 'a - это параметр времени жизни. Он указывает, что возвращаемая ссылка
// будет жить как минимум столько же, сколько и входные параметры.
fn lifetime_example<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {
      x
  } else {
      y
  }
}
// Эта функция сравнивает длины двух строковых срезов и возвращает более длинный.
// Времена жизни гарантируют, что возвращаемая ссылка действительна не дольше, чем входные ссылки.

// 5. Структуры и методы
#[derive(Debug)] // Позволяет использовать {:?} для вывода структуры
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  // &self - это сокращение для self: &Self, где Self - тип текущей структуры (Rectangle)
  fn area(&self) -> u32 {
      self.width * self.height
  }

  // Ассоциированная функция (не метод, так как не принимает self)
  fn new(width: u32, height: u32) -> Rectangle {
      Rectangle { width, height }
  }
}

// 6. Перечисления и сопоставление с образцом
enum Color {
  Red,
  Green,
  Blue,
  RGB(u8, u8, u8), // Перечисление может содержать данные
}

fn print_color(color: Color) {
  // match сопоставляет значение со всеми возможными вариантами
  match color {
      Color::Red => println!("Red"),
      Color::Green => println!("Green"),
      Color::Blue => println!("Blue"),
      Color::RGB(r, g, b) => println!("RGB({}, {}, {})", r, g, b),
  }
}

// 7. Обработка ошибок с Result и Option
fn divide(a: f64, b: f64) -> Result<f64, String> {
  if b == 0.0 {
      Err(String::from("Division by zero")) // Возвращаем ошибку
  } else {
      Ok(a / b) // Возвращаем результат
  }
}

fn option_example(x: Option<i32>) {
  match x {
      Some(i) => println!("Got an integer: {}", i),
      None => println!("No integer!"),
  }
}

// 8. Обобщенные типы и трейты
trait Printable {
  fn format(&self) -> String;
}

impl Printable for i32 {
  fn format(&self) -> String {
      format!("i32: {}", self)
  }
}

impl Printable for String {
  fn format(&self) -> String {
      format!("String: {}", self)
  }
}

// T: Printable означает, что T должен реализовывать трейт Printable
fn print_formatted<T: Printable>(item: T) {
  println!("{}", item.format());
}

// 9. Слайсы (Slices)
fn first_word(s: &str) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
      if item == b' ' { // b' ' - это байтовое представление пробела
          return &s[0..i]; // Возвращаем слайс от начала до найденного пробела
      }
  }

  &s[..] // Если пробел не найден, возвращаем весь слайс
}

// 10. Замыкания (Closures)
fn closures_example() {
  // Замыкание - это анонимная функция, которая может захватывать переменные из окружения
  let add_one = |x: i32| x + 1;
  println!("5 + 1 = {}", add_one(5));

  let numbers = vec![1, 2, 3, 4, 5];
  // Здесь мы используем замыкание в методе filter
  let even_numbers: Vec<i32> = numbers.into_iter().filter(|&x| x % 2 == 0).collect();
  println!("Even numbers: {:?}", even_numbers);
}

// Главная функция
fn main() {
  println!("1. Основные типы данных и переменные:");
  basic_types();

  println!("\n2. Управление потоком выполнения:");
  control_flow(10);

  println!("\n3. Владение и заимствование:");
  ownership_and_borrowing();

  println!("\n4. Времена жизни:");
  let s1 = String::from("short");
  let s2 = String::from("longer");
  let result = lifetime_example(&s1, &s2);
  println!("Более длинная строка: {}", result);

  println!("\n5. Структуры и методы:");
  let rect = Rectangle::new(5, 10);
  println!("Прямоугольник: {:?}", rect);
  println!("Площадь прямоугольника: {}", rect.area());

  println!("\n6. Перечисления и сопоставление с образцом:");
  print_color(Color::Red);
  print_color(Color::Green);
  print_color(Color::Blue);
  print_color(Color::RGB(255, 0, 0));

  println!("\n7. Обработка ошибок с Result и Option:");
  match divide(10.0, 2.0) {
      Ok(result) => println!("10 / 2 = {}", result),
      Err(e) => println!("Ошибка: {}", e),
  }
  option_example(Some(42));
  option_example(None);

  println!("\n8. Обобщенные типы и трейты:");
  print_formatted(42);
  print_formatted(String::from("Hello, Rust!"));

  println!("\n9. Слайсы:");
  let s = String::from("hello world");
  let word = first_word(&s);
  println!("Первое слово: {}", word);

  println!("\n10. Замыкания:");
  closures_example();
}
