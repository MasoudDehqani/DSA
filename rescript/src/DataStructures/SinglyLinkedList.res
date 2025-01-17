// type rec singlyListNode<'a> = {
//   value: 'a,
//   next: option<singlyListNode<'a>>,
// }

// let makeNode = (val: 'a) => {
//   value: val,
//   next: None,
// }

// let rec append = (l: singlyListNode<'a>, newVal: 'a) => {
//   let newNext = switch l.next {
//   | Some(n) => append(n, newVal)
//   | None => {value: newVal, next: None}
//   }

//   {value: l.value, next: Some(newNext)}
// }

// let sampleSinglyLinkedList = makeNode(1)->append(2)->append(3)->append(4)
// let sampleSinglyLinkedList = makeNode(1)->append(2)->append(3)->append(4)->deleteByIndex(1)

// let rec readByIndex = (l: singlyListNode<'a>, index: int, ~currentIndex: int=0) => {
//   switch (l.next, currentIndex < index) {
//   | (Some(n), true) => readByIndex(n, index, ~currentIndex={currentIndex + 1})
//   | (None, true) => None
//   | (_, false) =>
//     if currentIndex === index {
//       Some(l.value)
//     } else {
//       None
//     }
//   }
// }

// let sampleSinglyLinkedList = makeNode(1)->append(2)->append(3)->append(4)->readByIndex(0)

// let rec search = (l: singlyListNode<'a>, val: 'a, ~currentIndex=0) => {
//   switch (l.next, l.value === val) {
//   | (Some(n), false) => search(n, val, ~currentIndex={currentIndex + 1})
//   | (None, false) => None
//   | (_, true) => Some(currentIndex)
//   }
// }

// let sampleSinglyLinkedList = makeNode(1)->append(2)->append(3)->append(4)->search(3)

// let rec insertAtIndex = (l: singlyListNode<'a>, index: int, newVal: 'a, ~currentIndex=0) => {
//   let (newHead, shouldReArrange) = switch (index === 0, currentIndex === index - 1, l.next) {
//   | (true, _, _) => ({value: newVal, next: Some(l)}, false)
//   | (false, false, Some(n)) => (
//       insertAtIndex(n, index, newVal, ~currentIndex={currentIndex + 1}),
//       true,
//     )
//   | (false, true, _) => ({value: newVal, next: l.next}, true)
//   | (false, false, None) => (l, false)
//   }

//   if !shouldReArrange {
//     newHead
//   } else {
//     {value: l.value, next: Some(newHead)}
//   }
// }

// let rec insertAtIndex = (l: singlyListNode<'a>, index: int, newVal: 'a, ~currentIndex=0) => {
//   if index === 0 {
//     {value: newVal, next: Some(l)}
//   } else {
//     switch (currentIndex === index - 1, l.next) {
//     | (true, _) => {
//         value: l.value,
//         next: Some({value: newVal, next: l.next}),
//       }
//     | (false, Some(n)) => {
//         value: l.value,
//         next: Some(insertAtIndex(n, index, newVal, ~currentIndex={currentIndex + 1})),
//       }

//     | (false, None) => l
//     }
//   }
// }

// let sampleSinglyLinkedList = makeNode(1)->append(2)->append(4)->insertAtIndex(2, 3)

// let rec deleteByIndex = (l: singlyListNode<'a>, index: int, ~currentIndex=0) => {
//   if index === 0 {
//     switch l.next {
//     | Some(n) => n
//     | None => {value: -1, next: None}
//     }
//   } else {
//     switch (currentIndex === index - 1, l.next) {
//     | (false, Some(n)) => {
//         value: l.value,
//         next: Some(deleteByIndex(n, index, ~currentIndex={currentIndex + 1})),
//       }

//     | (false, None) => l

//     | (true, Some(n)) => {value: l.value, next: n.next}

//     | (true, None) => l
//     }
//   }
// }

// let sampleSinglyLinkedList = makeNode(1)->append(2)->append(3)->append(4)->deleteByIndex(0)

type rec listNode<'a> = {value: 'a, next: option<listNode<'a>>}
type rec singlyLinkedList<'a> = Empty | Node(listNode<'a>)

let makeNode = (val: 'a): listNode<'a> => {
  value: val,
  next: None,
}

let append = (lst: singlyLinkedList<'a>, newValue: 'a): singlyLinkedList<'a> => {
  let rec appendByValueHelper = (lst: singlyLinkedList<'a>, newValue: 'a) => {
    switch lst {
    | Empty => {value: newValue, next: None}
    | Node(node) =>
      switch node.next {
      | Some(nxt) => {value: node.value, next: Some(appendByValueHelper(Node(nxt), newValue))}
      | None => {value: node.value, next: Some({value: newValue, next: None})}
      }
    }
  }

  Node(appendByValueHelper(lst, newValue))
}

let rec readByIndex = (lst: singlyLinkedList<'a>, index: int, ~currentIndex=0): option<'a> => {
  switch lst {
  | Empty => None
  | Node(node) =>
    switch (currentIndex === index, node.next) {
    | (false, Some(nxt)) => readByIndex(Node(nxt), index, ~currentIndex={currentIndex + 1})
    | (false, None) => None
    | (true, _) => Some(node.value)
    }
  }
}

let rec search = (lst: singlyLinkedList<'a>, searchValue: 'a, ~currentIndex=0): option<int> => {
  switch lst {
  | Empty => None
  | Node(node) =>
    switch (searchValue === node.value, node.next) {
    | (false, Some(nxt)) => search(Node(nxt), searchValue, ~currentIndex={currentIndex + 1})
    | (false, None) => None
    | (true, _) => Some(currentIndex)
    }
  }
}

let insertAtIndex = (lst: singlyLinkedList<'a>, index: int, newValue: 'a) => {
  let rec insertAtIndexHelper = (
    lst: singlyLinkedList<'a>,
    index: int,
    newValue: 'a,
    currentIndex,
  ) => {
    switch lst {
    | Empty => {value: newValue, next: None}
    | Node(node) if index === 0 => {value: newValue, next: Some(node)}
    | Node(node) =>
      switch (currentIndex === index - 1, node.next) {
      | (false, Some(nxt)) => {
          value: node.value,
          next: Some(insertAtIndexHelper(Node(nxt), newValue, index, currentIndex + 1)),
        }
      | (false, None) => node
      | (true, _) => {value: node.value, next: Some({value: newValue, next: node.next})}
      }
    }
  }

  Node(insertAtIndexHelper(lst, newValue, index, 0))
}

let deleteByIndex = (lst: singlyLinkedList<'a>, index: int) => {
  let rec deleteByIndexHelper = (node, index, currentIndex) => {
    switch (currentIndex === index - 1, node.next) {
    | (false, Some(nxt)) => {
        value: node.value,
        next: Some(deleteByIndexHelper(nxt, index, currentIndex + 1)),
      }
    | (false, None) => node
    | (true, Some(nxt)) => {value: node.value, next: nxt.next}
    | (true, None) => node
    }
  }

  switch lst {
  | Empty => Empty
  | Node({value: _, next: Some(nxt)}) if index === 0 => Node({value: nxt.value, next: nxt.next})
  | Node({value: _, next: None}) if index === 0 => Empty
  | _ if index < 0 => lst
  | Node(node) => Node(deleteByIndexHelper(node, index, 0))
  }
}
