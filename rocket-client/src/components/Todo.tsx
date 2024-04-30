import { useCallback } from "react"
import { TodoProps } from "../utils/types"
import CheckBox from "./checkbox/CheckBox"
import cn from "../utils/cn"
import { X } from "lucide-react"
import useTodo from "../hooks/useTodo"
import { useSortable } from "@dnd-kit/sortable"
import { CSS } from "@dnd-kit/utilities"
import { toast } from "react-toastify"

interface TodoComp {
    todo: TodoProps
}

const Todo: React.FC<TodoComp> = ({ todo }) => {
    const { updateTodo: { trigger: triggerUpdate }, deleteTodo: { trigger: triggerDelete, isMutating: isDeleting } } = useTodo();
    const { attributes, listeners, setNodeRef, transform, transition } = useSortable({ id: todo._id.$oid });

    const style = {
        transition,
        transform: CSS.Transform.toString(transform)
    }

    const handleCheck = useCallback((todo: TodoProps) => {
        const updateTodo = { ...todo, completed: !todo.completed }


        triggerUpdate({ id: todo._id.$oid, data: updateTodo }, {
            optimisticData: (currentData) => {
                return currentData.map((ele: { _id: { $oid: string } }) => ele._id.$oid === todo._id.$oid ? updateTodo : ele)
            },
            onSuccess() {
                toast.info(updateTodo.completed ? "Task completed" : "Task Unchecked")
            }
        })
    }, [triggerUpdate])

    const handleDelete = useCallback((e: React.MouseEvent<HTMLButtonElement, MouseEvent>, id: string) => {
        e.stopPropagation()
        triggerDelete({ id }, {
            optimisticData: (currentData) => {
                return currentData.filter((ele: { _id: { $oid: string } }) => ele._id.$oid !== id)
            },
            onSuccess() {
                toast.error("Deleted your task")
            }
        })
    }, [triggerDelete])

    return (
        <article style={{ ...style, touchAction: 'none' }}
            className={cn("bg-white/10 px-2 py-2 rounded cursor-pointer group/todo", "flex justify-start flex-row items-center gap-3")}>
            <CheckBox handleCheck={useCallback(() => handleCheck(todo), [handleCheck, todo])} checked={todo.completed} />
            <div ref={setNodeRef} {...attributes} {...listeners} className="flex-1">
                <h4 className={cn("text-gray-200 flex-1", todo.completed && "line-through text-gray-200/50")}>{todo.body}</h4>
            </div>
            <button type="button" onClick={(e) => handleDelete(e, todo._id.$oid)} disabled={isDeleting} className="z-10 group/delete">
                <X className={cn("text-white opacity-0 group-hover/todo:opacity-100 group-disabled/delete:cursor-not-allowed group-disabled/delete:text-red-700")} />
            </button>
        </article>
    )
}

export default Todo