(* open Algorithms.Factorial *)
open DataStructures
open SinglyLinkedList

(* let () = 5 |> factorial |> string_of_int |> print_endline *)
(* let my_list: int SinglyLinkedList.t = Node (1, Node (2, Node (3, Nil))) *)
(* let my_stack : int Stack.t = Entry (1, Entry (2, Entry (3, Entry (4, Nil)))) *)

(* let v = SinglyLinkedList.read 1 my_list |> Option.value ~default:1000 *)
(* let v2 = SinglyLinkedList.search 2 my_list |> Option.value ~default:1000 *)
(* let v3 = SinglyLinkedList.search 1 my_list |> Option.value ~default:2000 *)
(* let v4 = Stack.push 5 my_stack *)

(* let () = v |> string_of_int |> print_endline *)
(* let () = v2 |> string_of_int |> print_endline *)
(* let () = v3 |> string_of_int |> print_endline *)

let my_list = Node (1, Node (2, Node (3, Nil)))

(* let my_list2 = Nil
let my_list3 = Node (1, Node (2, Node (3, Node (3, Node (4, Nil)))))
let my_list4 = Node (1, Nil)
let my_list5 = Node (1, Node (1, Node (2, Node (3, Node (4, Nil))))) *)

let () = prepend 0 my_list |> display
let () = append 4 my_list |> display
let () = insert 4 3 my_list |> display

(* let rec print_stack (stack : int Stack.t) =
  match stack with
  | Nil -> print_endline "Nil"
  | Entry (top, rest) ->
      top |> string_of_int |> print_endline;
      print_stack rest *)

(* let () = print_stack v4 *)
