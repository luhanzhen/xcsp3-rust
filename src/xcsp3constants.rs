/*=============================================================================
* parser for CSP instances represented in XCSP3 Format
*
* Copyright (c) 2023 xcsp.org (contact @ xcsp.org)
*
* Permission is hereby granted, free of charge, to any person obtaining a copy
* of this software and associated documentation files (the "Software"), to deal
* in the Software without restriction, including without limitation the rights
* to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
* copies of the Software, and to permit persons to whom the Software is
* furnished to do so, subject to the following conditions:
*
* The above copyright notice and this permission notice shall be included in
* all copies or substantial portions of the Software.
*
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
* IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
* FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
* AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
* LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
* OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
* THE SOFTWARE.
*=============================================================================
*/


/**
 * <p>@project_name: XCSP3-Rust
 * <p/>
 * <p>@author: luhanzhen
 * <p/>
 * <p>@date: 2023/6/30
 * <p/>
 * <p>@time: 13:44
 * <p/>
 * <p>@this_file_name:XCSP3Constants
 * <p/>
 */

pub mod xcsp3_core
{
    enum InstanceType {
        CPS,
        COP,
    }

    enum ConstraintType {
        UNKNOWN,
        EXTENSION,
        INTENSION,
        ALLDIFF,
        ALLEQUAL,
        SUM,
        ORDERED,
        COUNT,
        NVALUES,
        CARDINALITY,
        MAXIMUM,
        MINIMUM,
        ELEMENT,
        ELEMENTMATRIX,
        NOOVERLAP,
        STRETCH,
        LEX,
        CHANNEL,
        REGULAR,
        MDD,
        CUMULATIVE,
        INSTANTIATION,
        CIRCUIT,
        CLAUSE,
        PRECEDENCE,
        BINPACKING,
        FLOW,
        KNAPSACK,
        MINARG,
        MAXARG,
    }

    enum OrderType {
        LE,
        LT,
        GE,
        GT,
        IN,
        EQ,
        NE,
    }

    enum Tag {
        UnknownTag,
        ListTag,
        FunctionalTag,
        ValuesTag,
        ValueTag,
        ConditionTag,
        IndexTag,
        LengthsTag,
    }

    enum RankType {
        ANY,
        FIRST,
        LAST,
    }

    enum ObjectiveGoal {
        MINIMIZE,
        MAXIMIZE,
    }

    enum ExpressionObjective {
        ExpressionZero,
        SumZero,
        ProductZero,
        MinimumZero,
        MaximumZero,
        NvaluesZero,
        LexZero,
    }
}
