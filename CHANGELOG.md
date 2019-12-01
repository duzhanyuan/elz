# Change log

### 0.1.0: in progress

#### Syntax

* global variable
    ```elz
    x: int = 1;
    ```
* global function definition
    ```elz
    main(): void {}
    // block body
    foo(): void {}
    // expression body
    bar(): int = 1;
    // or we can use block
    bar(): int {
      return 1;
    }
    ```
* local variable
    ```elz
    main(): void {
      x: int = 1;
    }
    ```
* function call
    ```elz
    main(): void {
      println("hello, world");
    }
    ```
* string literal and template
    ```elz
    main(): void {
      x: int = 1;
      println("x = {x}");
    }
    ```
* List literal
    ```elz
    x: List[int] = [];
    ```

#### Type

* `void`
* `int`
* `string`
* `bool`
* `f64`
* `List[T]`
* function type, e.g. `(int, int): int`