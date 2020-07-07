package model.quantities.amounts

class AmountBinary(
    val bop: AmountBinaryOp, val left: Amount, val right: Amount
) : Amount {
    override fun toString(): String {
        return left.toString() + bop.symbol + right.toString()
    }
}