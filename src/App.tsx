import { format } from 'path';
import { useEffect, useState } from 'react';
import './App.css';

function App() {
    const [text, setText] = useState<string>("");
    const expensesService: ExpensesService = staticExpensesService;
    const expenses: Expense[] = expensesService.fetchExpenses();
    // useEffect(() => {
    // const response = await fetch('/api');
    // const data = await response.text();
    // console.info(data);
    // setText(data);
    // }, []);

    return (
        <div className="App">
            <header className="App-header">
                <table className="table-auto border-collapse w-full border">
                    <thead>
                        <tr>
                            <th>Date</th>
                            <th>Tag</th>
                            <th>Amount</th>
                        </tr>
                    </thead>
                    <tbody>
                        {expenses.map((expense) => (
                            <tr key={expense.id}>
                                <td>{formatDate(expense.date)}</td>
                                <td>{expense.tag}</td>
                                <td>{expense.amount}</td>
                            </tr>
                        ))}
                    </tbody>
                </table>
            </header>
        </div>
    );
}

interface ExpensesService {
    fetchExpenses(): Expense[]
}

interface Expense {
    id: number,
    tag: string,
    amount: number,
    date: Date
}

const staticExpensesService: ExpensesService = {
    fetchExpenses(): Expense[] {
        return [
            { id: 0, tag: "Grocery", amount: 100, date: new Date("2024-5-17") }
        ]
    }
}

function formatDate(date: Date): string {
    return [date.getDay(), date.getMonth(), date.getFullYear()].join('-');
}

export default App;
