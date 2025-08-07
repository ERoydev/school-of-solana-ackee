use std::fmt;

///-------------------------------------------------------------------------------
///
/// This is your calculator implementation task
/// to practice enums, structs, and methods.
///
/// Complete the implementation of the Calculator struct and its methods.
///
/// The calculator should support basic arithmetic
/// operations (addition, subtraction, multiplication)
/// with overflow protection and maintain a history
/// of operations.
///
/// Tasks:
/// 1. Implement the OperationType enum methods
/// 2. Implement the Operation struct constructor
/// 3. Implement all Calculator methods
///
///-------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub enum OperationType {
    Addition,
    Subtraction,
    Multiplication,
}

impl OperationType {
    // TODO: Return the string representation of the operation sign
    // Addition -> "+", Subtraction -> "-", Multiplication -> "*"
    pub fn get_sign(&self) -> &str {
        match self {
            OperationType::Addition => "+",
            OperationType::Subtraction => "-",
            OperationType::Multiplication => "*",
        }
    }

    // TODO: Perform the operation on two i64 numbers with overflow protection
    // Return Some(result) on success, None on overflow
    //
    // Example: OperationType::Multiplication.perform(x, y)
    pub fn perform(&self, x: i64, y: i64) -> Option<i64> {
        match self {
            OperationType::Addition => x.checked_add(y),
            OperationType::Multiplication => x.checked_mul(y),
            OperationType::Subtraction => x.checked_sub(y),
        }
    }
}

impl fmt::Display for OperationType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_sign())
    }
}
#[derive(Clone, Debug)]
pub struct Operation {
    pub first_num: i64,
    pub second_num: i64,
    pub operation_type: OperationType,
}

impl Operation {
    // TODO: Create a new Operation with the given parameters
    pub fn new(first_num: i64, second_num: i64, operation_type: OperationType) -> Self {
        Operation {
            first_num,
            second_num,
            operation_type,
        }
    }
}

pub struct Calculator {
    pub history: Vec<Operation>,
}

impl Calculator {
    // TODO: Create a new Calculator with empty history
    pub fn new() -> Self {
        Calculator {
            history: Vec::new(),
        }
    }

    // TODO: Perform addition and store successful operations in history
    // Return Some(result) on success, None on overflow
    pub fn addition(&mut self, x: i64, y: i64) -> Option<i64> {
        let result = x.checked_add(y);
        let operation = Operation::new(x, y, OperationType::Addition);
        self.history.push(operation);
        return result;
    }

    // TODO: Perform subtraction and store successful operations in history
    // Return Some(result) on success, None on overflow
    pub fn subtraction(&mut self, x: i64, y: i64) -> Option<i64> {
        let result = x.checked_sub(y);
        let operation = Operation::new(x, y, OperationType::Subtraction);
        self.history.push(operation);
        return result;
    }

    // TODO: Perform multiplication and store successful operations in history
    // Return Some(result) on success, None on overflow
    pub fn multiplication(&mut self, x: i64, y: i64) -> Option<i64> {
        let result = x.checked_mul(y);
        let operation = Operation::new(x, y, OperationType::Multiplication);
        self.history.push(operation);
        return result;
    }

    // TODO: Generate a formatted string showing all operations in history
    // Format: "index: first_num operation_sign second_num = result\n"
    //
    // Example: "0: 5 + 3 = 8\n1: 10 - 2 = 8\n"
    pub fn show_history(&self) -> String {
        let mut formatted_string = String::new();

        for i in 0..self.history.len() {
            let current_operation = self.history.get(i);
            if let Some(item) = current_operation {
                let operation_result = item.operation_type.perform(item.first_num, item.second_num).unwrap();
                // let result = current_operation
                let message = format!(
                    "{i}: {n1} {o} {n2} = {r}\n",
                    i = i,
                    n1 = item.first_num,
                    o = item.operation_type,
                    n2 = item.second_num,
                    r = operation_result
                );

                formatted_string.push_str(&message);
            }
        }
        formatted_string
    }

    // TODO: Repeat an operation from history by index
    // Add the repeated operation to history and return the result
    // Return None if the index is invalid
    pub fn repeat(&mut self, operation_index: usize) -> Option<i64> {
        let chosen_operation = self.history.get(operation_index);
        let mut result: Option<i64> = Option::None;

        if let Some(operation) = chosen_operation {
            result = operation.operation_type.perform(operation.first_num, operation.second_num);
            let new_operation = Operation::new(operation.first_num, operation.second_num, operation.operation_type.clone());
            self.history.push(new_operation);
            
        } 
        return result
    }

    // TODO: Clear all operations from history
    pub fn clear_history(&mut self) {
        self.history.clear();
    }
}
