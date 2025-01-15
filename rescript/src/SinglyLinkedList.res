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

// let rec deleteByIndex = (l: singlyListNode<'a>, index: int, ~currentIndex=0) => {
//   let newNext = if currentIndex < index {
//     switch l.next {
//     | Some(n) => deleteByIndex(n, index, ~currentIndex={currentIndex + 1})
//     | None => Some(l)
//     }
//   } else if currentIndex === index {
//     l.next
//   } else {
//     Some(l)
//   }

//   {value: l.value, next: newNext}

//   // let newNext = switch l.next {
//   // | Some(n) =>
//   //   if index === currentIndex {
//   //     {value: n.value, next: n.next}
//   //   } else {
//   //     deleteByIndex(n, index, ~currentIndex={currentIndex + 1})
//   //   }

//   // | None => l
//   // }

//   // if index === currentIndex {
//   //   {value: newNext.value, next: newNext.next}
//   // } else {
//   //   {value: l.value, next: Some(newNext)}
//   // }
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

let rec insertAtIndex = (l: singlyListNode<'a>, index: int, newVal: 'a, ~currentIndex=0) => {
  if index === 0 {
    {value: newVal, next: Some(l)}
  } else {
    switch (currentIndex === index - 1, l.next) {
    | (true, _) => {
        value: l.value,
        next: Some({value: newVal, next: l.next}),
      }
    | (false, Some(n)) => {
        value: l.value,
        next: Some(insertAtIndex(n, index, newVal, ~currentIndex={currentIndex + 1})),
      }

    | (false, None) => l
    }
  }
}

let sampleSinglyLinkedList = makeNode(1)->append(2)->append(4)->insertAtIndex(2, 3)
