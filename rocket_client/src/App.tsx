import TodosContainer from "./components/TodosContainer";
import Todo from "./components/Todo";
import AddButton from "./components/AddButton";
import useTodo from "./hooks/useTodo";

function App() {
  const { todos } = useTodo();

  return (
    <main className="relative min-w-[300px] min-h-[70vh] shadow-md rounded-lg bg-zinc-900 overflow-hidden space-y-4">
      <header className="bg-teal-500 px-4 py-2 flex justify-center">
        <h1 className="font-semibold">Rocket Todo</h1>
      </header>
      <TodosContainer className="h-[40vh] overflow-auto custom-scroll">
        {
          todos ?
            todos.map(todo => <Todo key={todo._id.$oid} todo={todo} />)
            : <h3>No Todos</h3>
        }
      </TodosContainer>
      <AddButton className="absolute z-10 bottom-2 right-2" />
    </main>
  )
}

export default App
