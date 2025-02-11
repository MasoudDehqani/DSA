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

let rec insert value index lst =
  match (lst, index = 0) with
  | Nil, false -> Nil
  | Nil, true -> Node (value, Nil)
  | Node (head, tail), false -> Node (head, insert value (index - 1) tail)
  | Node (head, tail), true -> Node (value, Node (head, tail))

let rec delete index = function
  | Nil -> Nil
  | Node (head, tail) ->
      if index = 0 then tail else Node (head, delete (index - 1) tail)

let rec length = function
  | Nil -> 0
  | Node (_, tail) -> length tail + 1

let rec display_aux lst acc =
  match lst with
  | Nil -> acc ^ "Nil"
  | Node (head, tail) ->
      let string_of_head = head |> string_of_int in
      let new_acc = acc ^ string_of_head ^ " -> " in
      display_aux tail new_acc

let display lst = print_endline (display_aux lst "")

let rec reverse_aux lst acc =
  match lst with
  | Nil -> acc
  | Node (head, tail) -> reverse_aux tail @@ Node (head, acc)

let reverse lst = reverse_aux lst Nil

let rec map fn = function
  | Nil -> Nil
  | Node (head, tail) -> Node (fn head, map fn tail)

let rec fold_left fn acc = function
  | Nil -> acc
  | Node (head, tail) -> fold_left fn (fn acc head) tail

let rec fold_right fn lst acc =
  match lst with
  | Nil -> acc
  | Node (head, tail) -> fn head (fold_right fn tail acc)

let rec filter fn = function
  | Nil -> Nil
  | Node (head, tail) ->
      let is_true = fn head in
      if is_true then Node (head, filter fn tail) else filter fn tail
