# Tensor EBNF Finalization

This document finalizes the Tensor syntax for parsing and type references.

## 1. Type Syntax
Tensor types are written as:
```
Tensor<T, [d1, d2, ...]>
```

Where:
- `T` is any type reference (primitive, struct, alias, etc.)
- Each dimension `d` is:
  - integer literal (e.g., `224`)
  - identifier (symbolic dimension)
  - `_` for unknown

## 2. Literal Syntax
Tensor literals use the `tensor` keyword and nested array rows:
```
tensor[[1, 2], [3, 4]]
```

## 3. Grammar Snippet (EBNF)
```
tensor_type       = "Tensor" , "<" , type_ref , "," , shape_ref , ">" ;
shape_ref         = "[" , shape_dim , { "," , shape_dim } , "]" ;
shape_dim         = int_lit | identifier | "_" ;

tensor_lit        = "tensor" , "[" , tensor_rows , "]" ;
tensor_rows       = tensor_row , { "," , tensor_row } ;
tensor_row        = "[" , [ expr , { "," , expr } ] , "]" ;
```

## 4. Examples
```
let img: Tensor<Float, [224, 224, 3]> = ...;
let batch: Tensor<Float, [N, 224, 224, 3]> = ...;
let unknown: Tensor<Float, [_, 128]> = ...;
```

