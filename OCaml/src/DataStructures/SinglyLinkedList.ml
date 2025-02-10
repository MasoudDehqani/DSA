type 'a t =
  | Nil
  | Node of 'a * 'a t

let rec read index lst =
  if index < 0 then None
  else
    match lst with
    | Nil -> None
    | Node (head, tail) ->
        if index = 0 then Some head else read (index - 1) tail

let rec search_aux value current_index = function
  | Nil -> None
  | Node (head, tail) ->
      if head = value then Some current_index
      else search_aux value (current_index + 1) tail

let search value lst = search_aux value 0 lst

let rec append value = function
  | Nil -> Node (value, Nil)
  | Node (head, tail) -> Node (head, append value tail)

let prepend value lst = Node (value, lst)

let rec insert value index = function
  | Nil -> if index = 0 then Node (value, Nil) else Nil
  | Node (head, tail) ->
      if index = 0 then Node (value, Node (head, tail))
      else Node (head, insert value (index - 1) tail)

let rec delete index = function
  | Nil -> Nil
  | Node (head, tail) ->
      if index = 0 then tail else Node (head, delete (index - 1) tail)

let rec length = function
  | Nil -> 0
  | Node (_head, tail) -> length tail + 1

let rec display_aux lst acc =
  match lst with
  | Nil -> acc ^ "Nil"
  | Node (head, tail) ->
      let new_acc = acc ^ (head |> string_of_int) ^ " -> " in
      display_aux tail new_acc

let display lst = print_endline (display_aux lst "")
