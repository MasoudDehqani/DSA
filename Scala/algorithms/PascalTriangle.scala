def pascal(c: Int, r: Int): Int =
  if r < 2 || (c < 1 || c >= r)
  then 1
  else pascal(c, r - 1) + pascal(c - 1, r - 1)

// def pascalMemo(c: Int, r: Int): Int =
//   val memo = collection.mutable.Map[String, Int]()

//   def aux(column: Int, row: Int): Int =
//     val keyTopLeft = (column - 1).toString() + (row - 1).toString()
//     val keyTopRight = column.toString() + (row - 1).toString()
//     if row < 2 || (column < 1 || column >= row)
//     then 1
//     else
//       memo.getOrElseUpdate(keyTopRight, aux(column, row - 1)) + memo
//         .getOrElseUpdate(keyTopLeft, aux(column - 1, row - 1))

//   aux(c, r)

def pascalMemo(c: Int, r: Int): Int =
  val memo = collection.mutable.Map[(Int, Int), Int]()

  def aux(column: Int, row: Int): Int =
    memo.getOrElseUpdate(
      (column, row),
      if row < 2 || column < 1 || column >= row then 1
      else aux(column, row - 1) + aux(column - 1, row - 1)
    )

    aux(c, r)

/** Exercise 2
  */
def balance(chars: List[Char]): Boolean =
  def aux(cs: List[Char], open: Int): Boolean =
    cs match
      case Nil      => open == 0
      case '(' :: t => aux(t, open + 1)
      case ')' :: t => open > 0 && aux(t, open - 1)
      case _ :: t   => aux(t, open)
  aux(chars, 0)
