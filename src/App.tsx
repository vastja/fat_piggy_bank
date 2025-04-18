import { useEffect, useState } from 'react';
import './App.css';
import { Chart as ChartJS, ArcElement, Tooltip } from 'chart.js';
import { Pie } from 'react-chartjs-2';

ChartJS.register(ArcElement, Tooltip);

function App() {
    const [expenses, setExpenses] = useState<number[]>([]);
    useEffect(() => {
        const fetchExpenses = async () => {
            const expensesService: ExpensesService = staticExpensesService;
            const expenses: Expense[] = await expensesService.fetchExpenses(true);
            setExpenses(expenses.map(x => x.amount));
        }
        fetchExpenses();
    }, []);
    return (
        <div className="App">
            <header className="App-header">
                <div style={{ width: '25%', height: '25%' }}>
                    <Pie data={{ datasets: [{ data: expenses }] }} />
                </div>
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
