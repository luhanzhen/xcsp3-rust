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
