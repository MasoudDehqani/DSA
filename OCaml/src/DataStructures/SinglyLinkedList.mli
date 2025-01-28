type 'a t =
  | Nil
  | Node of 'a * 'a t

val read : int -> 'a t -> 'a option
val search : 'a -> 'a t -> int option
(* val append: 'a -> 'a t -> 'a t
  val insert: 'a -> int -> 'a t -> 'a t
  val delete: int -> 'a t -> 'a t *)
