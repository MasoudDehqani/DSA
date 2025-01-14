type rec singlyListNode<'a> = {
  value: 'a,
  next: option<singlyListNode<'a>>,
}

let makeNode = (val: 'a) => {
  value: val,
  next: None,
}

let rec append = (l: singlyListNode<'a>, newVal: 'a) => {
  let newNext = switch l.next {
  | Some(n) => append(n, newVal)
  | None => {value: newVal, next: None}
  }

  {value: l.value, next: Some(newNext)}
}

let sampleSinglyLinkedList = makeNode(1)->append(2)->append(3)->append(4)
