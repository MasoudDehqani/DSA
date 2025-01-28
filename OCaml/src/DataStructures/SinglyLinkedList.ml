type 'a t = Nil | Node of 'a * 'a t

let rec read index = function
  | Nil -> None
  | Node (head, tail) -> if index = 0 then Some head else read (index - 1) tail

let rec search_aux value ?(current_index = 0) lst =
  match lst with
  | Nil -> None
  | Node (head, tail) ->
      if head = value then Some current_index
      else search_aux value tail ~current_index:(current_index + 1)

let search value lst = search_aux value lst ~current_index:0
