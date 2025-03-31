import { useEffect, useState } from 'react';
import './App.css';

function App() {
    return (
        <div className="App">
            <header className="App-header">
                <Expenses grouped={false} />
                <Expenses grouped={true} />
            </header>
        </div>
    );
}

interface ExpensesProps {
    grouped: boolean
}

const Expenses: React.FC<ExpensesProps> = ({ grouped }) => {
    const [expenses, setExpenses] = useState<Expense[]>([]);
    useEffect(() => {
        const fetchExpenses = async () => {
            const expensesService: ExpensesService = staticExpensesService;
            const expenses: Expense[] = await expensesService.fetchExpenses(grouped);
            setExpenses(expenses);
        }
        fetchExpenses();
    }, []);
    return (
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
    );
}

interface ExpensesService {
    fetchExpenses(grouped: boolean): Promise<Expense[]>
}

interface Expense {
    id: number,
    tag: string,
    amount: number,
    date: Date
}

const staticExpensesService: ExpensesService = {
    async fetchExpenses(grouped: boolean): Promise<Expense[]> {
        const response = await fetch(`/api/expenses?group=${grouped}`);
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
