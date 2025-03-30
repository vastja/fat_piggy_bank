import { useEffect, useState } from 'react';
import './App.css';

function App() {
    const [expenses, setExpenses] = useState<Expense[]>([]);
    useEffect(() => {
        const fetchExpenses = async () => {
            const expensesService: ExpensesService = staticExpensesService;
            const expenses: Expense[] = await expensesService.fetchExpenses();
            setExpenses(expenses);
        }
        fetchExpenses();
    }, []);

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
    fetchExpenses(): Promise<Expense[]>
}

interface Expense {
    id: number,
    tag: string,
    amount: number,
    date: Date
}

const staticExpensesService: ExpensesService = {
    async fetchExpenses(): Promise<Expense[]> {
        const response = await fetch('/api');
        const data = await response.text();
        return JSON.parse(data).map((x: any) => ({
            ...x,
            date: new Date(x.date)
        }));
    }
}

function formatDate(date: Date): string {
    console.log(date);
    return [date.getDate(), date.getMonth() + 1, date.getFullYear()].join('-');
}

export default App;
