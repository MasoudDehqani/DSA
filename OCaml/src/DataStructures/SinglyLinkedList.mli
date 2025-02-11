type 'a t =
  | Nil
  | Node of 'a * 'a t

val read : int -> 'a t -> 'a option
val search : 'a -> 'a t -> int option
val append : 'a -> 'a t -> 'a t
val prepend : 'a -> 'a t -> 'a t
val insert : 'a -> int -> 'a t -> 'a t
val delete : int -> 'a t -> 'a t
val length : 'a t -> int
val reverse : 'a t -> 'a t
val display : int t -> unit
val map : ('a -> 'b) -> 'a t -> 'b t
val fold_left : ('b -> 'a -> 'b) -> 'b -> 'a t -> 'b
val fold_right : ('a -> 'b -> 'b) -> 'a t -> 'b -> 'b
val filter : ('a -> bool) -> 'a t -> 'a t
