type 'a t =
  | Nil
  | Entry of 'a * 'a t

val pop: 'a t -> 'a t
val peek: 'a t -> 'a option
val push: 'a -> 'a t -> 'a t