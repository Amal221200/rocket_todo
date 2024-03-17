import TodosContainer from "./components/TodosContainer";
import Todo from "./components/Todo";
import AddButton from "./components/AddButton";
import useTodo from "./hooks/useTodo";
import { DndContext, DragEndEvent, KeyboardSensor, PointerSensor, TouchSensor, closestCorners, useSensor, useSensors } from "@dnd-kit/core";
import { TodoProps } from "./utils/types";
import { arrayMove, sortableKeyboardCoordinates } from "@dnd-kit/sortable";
const getPos = (tasks: TodoProps[], id: string) => tasks.findIndex((task) => task._id.$oid === id)

function App() {

  const { todos, changeOrder: { trigger: triggerChangeOrder } } = useTodo();

  const sensors = useSensors(useSensor(PointerSensor), useSensor(TouchSensor), useSensor(KeyboardSensor, {
    coordinateGetter: sortableKeyboardCoordinates
  }));
  const handleDragEnd = (event: DragEndEvent) => {
    const { active, over } = event

    if (active.id === over?.id) {
      return null
    }

    triggerChangeOrder({ id: active.id as string, replaceId: over?.id as string }, {
      optimisticData: (current) => {
        const activeInd = getPos(current, active.id as string)
        const overInd = getPos(current, over?.id as string)

        return arrayMove(current, activeInd, overInd)
      }
    })
  }
  return (
    <main className="relative min-w-[300px] min-h-[70vh] shadow-md rounded-lg bg-zinc-900 overflow-hidden space-y-4">
      <header className="bg-teal-500 px-4 py-2 flex justify-center">
        <h1 className="font-semibold">Rocket Todo</h1>
      </header>
      <DndContext onDragEnd={handleDragEnd} sensors={sensors} collisionDetection={closestCorners}>

        <TodosContainer className="h-[40vh] overflow-y-auto custom-scroll">
          {
            todos ?
              todos.map(todo => <Todo key={todo._id.$oid} todo={todo} />)
              : <h3>No Todos</h3>
          }
        </TodosContainer>
        <AddButton className="absolute z-10 bottom-2 right-2" />
      </DndContext>
    </main>
  )
}

export default App
