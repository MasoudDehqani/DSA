(* module type Stack = sig
  type 'a stack  
  val empty: 'a stack
  
  val push: 'a -> 'a stack -> 'a stack

  val peek: 'a stack -> 'a option
  
  val pop: 'a stack -> 'a option
end

module Stack: Stack = struct
  
  type 'a stack = 
    | Empty
    | Entry of 'a * 'a stack

  let empty = Empty
  
  let push v s = Entry (v, s)
  
  let peek = function
    | Empty -> None
    | Entry (v, _) -> Some(v)
  
  let pop = function
    | Empty -> None
    | Entry (v, _) -> Some(v)
end *)

type 'a t = Nil | Entry of 'a * 'a t

let pop = function
  | Nil -> Nil
  | Entry (_top, rest) -> rest

let peek = function
  | Nil -> None
  | Entry (top, _rest) -> Some top

let push v = function
  | Nil -> Entry (v, Nil)
  | Entry (top, rest) -> Entry (v, Entry (top, rest))
