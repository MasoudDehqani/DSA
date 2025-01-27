(* open Algorithms.Factorial *)
open DataStructures

(* let () = 5 |> factorial |> string_of_int |> print_endline *)
let my_list: int SinglyLinkedList.t = Node (1, Node (2, Node (3, Nil)))

let v = SinglyLinkedList.read 1 my_list |> Option.value ~default:1000
let () = v |> string_of_int |> print_endline