namespace AdventOfCode.Core.Numerics;

public static class Integer
{
    /// <summary>
    /// Finds the greatest common divisor of two BigInteger values.
    /// </summary>
    /// <param name="left">The first value.</param>
    /// <param name="right">The second value.</param>
    /// <returns>The greatest common divisor of left and right.</returns>
    public static int GreatestCommonDivisor(int left, int right)
    {
        while (true)
        {
            if (right == 0) return left;
            (left, right) = (right, left);
        }
    }
}