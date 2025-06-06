module SinglyLinkedListV1 = {
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

  let rec readByIndex = (l: singlyListNode<'a>, index: int, ~currentIndex: int=0) => {
    switch (l.next, currentIndex < index) {
    | (Some(n), true) => readByIndex(n, index, ~currentIndex={currentIndex + 1})
    | (None, true) => None
    | (_, false) =>
      if currentIndex === index {
        Some(l.value)
      } else {
        None
      }
    }
  }

  let rec search = (l: singlyListNode<'a>, val: 'a, ~currentIndex=0) => {
    switch (l.next, l.value === val) {
    | (Some(n), false) => search(n, val, ~currentIndex={currentIndex + 1})
    | (None, false) => None
    | (_, true) => Some(currentIndex)
    }
  }

  module InsertAtIndexV1 = {
    let rec insertAtIndex = (l: singlyListNode<'a>, index: int, newVal: 'a, ~currentIndex=0) => {
      let (newHead, shouldReArrange) = switch (index === 0, currentIndex === index - 1, l.next) {
      | (true, _, _) => ({value: newVal, next: Some(l)}, false)
      | (false, false, Some(n)) => (
          insertAtIndex(n, index, newVal, ~currentIndex={currentIndex + 1}),
          true,
        )
      | (false, true, _) => ({value: newVal, next: l.next}, true)
      | (false, false, None) => (l, false)
      }

      if !shouldReArrange {
        newHead
      } else {
        {value: l.value, next: Some(newHead)}
      }
    }
  }

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

  let rec deleteByIndex = (l: singlyListNode<'a>, index: int, ~currentIndex=0) => {
    if index === 0 {
      switch l.next {
      | Some(n) => n
      | None => {value: -1, next: None}
      }
    } else {
      switch (currentIndex === index - 1, l.next) {
      | (false, Some(n)) => {
          value: l.value,
          next: Some(deleteByIndex(n, index, ~currentIndex={currentIndex + 1})),
        }

      | (false, None) => l

      | (true, Some(n)) => {value: l.value, next: n.next}

      | (true, None) => l
      }
    }
  }
}

module SinglyLinkedListV2 = {
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
    let rec insertAtIndexHelper = (node: listNode<'a>, index: int, newValue: 'a, currentIndex) => {
      switch (currentIndex === index - 1, node.next) {
      | (false, Some(nxt)) => {
          value: node.value,
          next: Some(insertAtIndexHelper(nxt, index, newValue, currentIndex + 1)),
        }
      | (false, None) => node
      | (true, _) => {value: node.value, next: Some({value: newValue, next: node.next})}
      }
    }

    switch lst {
    | _ if index < 0 => lst
    | Empty => Node({value: newValue, next: None})
    | Node(node) if index === 0 => Node({value: newValue, next: Some(node)})
    | Node(node) => Node(insertAtIndexHelper(node, index, newValue, 0))
    }
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
}

type rec singlyLinkedList<'a> = Empty | Node('a, singlyLinkedList<'a>)

exception EmptyList(string)
exception OutOfBoundIndex(string)

let rec read = (lst: singlyLinkedList<'a>, index: int): option<'a> => {
  if index < 0 {
    raise(Invalid_argument("negative index"))
  } else {
    switch lst {
    | Empty => None
    | Node(head, tail) => index === 0 ? Some(head) : read(tail, index - 1)
    }
  }
}

let search = (lst: singlyLinkedList<'a>, val: 'a): option<int> => {
  let rec searchAux = (lst, val, currentIndex) => {
    switch lst {
    | Empty => None
    | Node(head, tail) => val === head ? Some(currentIndex) : searchAux(tail, val, currentIndex + 1)
    }
  }

  searchAux(lst, val, 0)
}

let rec append = (lst: singlyLinkedList<'a>, val: 'a): singlyLinkedList<'a> => {
  switch lst {
  | Empty => Node(val, Empty)
  | Node(head, tail) => Node(head, append(tail, val))
  }
}

let prepend = (lst: singlyLinkedList<'a>, val: 'a): singlyLinkedList<'a> => {
  Node(val, lst)
}

let insert = (lst: singlyLinkedList<'a>, val: 'a, index: int): singlyLinkedList<'a> => {
  let rec insertAux = (lst, val, index) => {
    switch lst {
    | Empty => index === 0 ? Node(val, Empty) : raise(OutOfBoundIndex("out of bound index"))
    | Node(head, tail) =>
      index === 0 ? Node(val, Node(head, tail)) : Node(head, insertAux(tail, val, index - 1))
    }
  }

  if index < 0 {
    raise(Invalid_argument("negative index"))
  } else {
    insertAux(lst, val, index)
  }
}

let delete = (lst: singlyLinkedList<'a>, index: int): singlyLinkedList<'a> => {
  let rec deleteAux = (lst, index) => {
    switch lst {
    | Empty => raise(OutOfBoundIndex("out of bound index"))
    | Node(head, tail) => index === 0 ? tail : Node(head, deleteAux(tail, index - 1))
    }
  }

  if index < 0 {
    raise(Invalid_argument("negative index"))
  } else if lst === Empty {
    raise(EmptyList("deletion from an empty list"))
  } else {
    deleteAux(lst, index)
  }
}

let rec pop = (lst: singlyLinkedList<'a>): singlyLinkedList<'a> => {
  switch lst {
  | Empty => raise(EmptyList("cannot pop from an empty list"))
  | Node(_head, Empty) => Empty
  | Node(head, tail) => Node(head, pop(tail))
  }
}

let rec peek = (lst: singlyLinkedList<'a>): option<'a> => {
  switch lst {
  | Empty => None
  | Node(head, Empty) => Some(head)
  | Node(_head, tail) => peek(tail)
  }
}

let reverse = (lst: singlyLinkedList<'a>): singlyLinkedList<'a> => {
  let rec reverseAux = (lst: singlyLinkedList<'a>, acc: singlyLinkedList<'a>) => {
    switch lst {
    | Empty => acc
    | Node(head, tail) => reverseAux(tail, Node(head, acc))
    }
  }

  reverseAux(lst, Empty)
}

let displayIntList = (lst: singlyLinkedList<int>): unit => {
  let rec displayAux = (lst, acc) => {
    switch lst {
    | Empty => acc ++ "Empty"
    | Node(head, tail) => displayAux(tail, acc ++ `${head->Int.toString} -> `)
    }
  }

  Console.log(displayAux(lst, ""))
}

let rec map = (lst: singlyLinkedList<'a>, fn: 'a => 'b): singlyLinkedList<'b> => {
  switch lst {
  | Empty => Empty
  | Node(head, tail) => Node(fn(head), map(tail, fn))
  }
}

let rec filter = (lst: singlyLinkedList<'a>, fn: 'a => bool): singlyLinkedList<'a> => {
  switch lst {
  | Empty => Empty
  | Node(head, tail) => fn(head) ? Node(head, filter(tail, fn)) : filter(tail, fn)
  }
}

let rec reduceLeft = (lst: singlyLinkedList<'a>, fn: ('b, 'a) => 'b, acc: 'b): 'b => {
  switch lst {
  | Empty => acc
  | Node(head, tail) => reduceLeft(tail, fn, fn(acc, head))
  }
}

let rec reduceRight = (lst: singlyLinkedList<'a>, fn: ('b, 'a) => 'b, acc: 'b): 'b => {
  switch lst {
  | Empty => acc
  | Node(head, tail) => fn(reduceRight(tail, fn, acc), head)
  }
}
