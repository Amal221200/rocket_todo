import React, { useCallback } from "react"
import { TodoProps } from "../utils/types"
import CheckBox from "./checkbox/CheckBox"
import cn from "../utils/cn"
import { X } from "lucide-react"
import useTodo from "../hooks/useTodo"

interface TodoComp {
    todo: TodoProps
}

const Todo: React.FC<TodoComp> = ({ todo }) => {
    const { updateTodo: { trigger: triggerUpdate }, deleteTodo: { trigger: triggerDelete } } = useTodo();

    const handleCheck = useCallback(() => {
        triggerUpdate({ id: todo._id.$oid, data: { ...todo, completed: !todo.completed } })
    }, [todo, triggerUpdate])

    const handleDelete = useCallback((e: React.MouseEvent<SVGSVGElement, MouseEvent>) => {
        e.stopPropagation()
        triggerDelete({ id: todo._id.$oid })
    }, [todo, triggerDelete])

    return (
        <article onClick={handleCheck} className="bg-white/10 px-2 py-2 rounded cursor-pointer group/todo">
            <div className="flex justify-start flex-row items-center gap-3">
                <CheckBox key={todo._id.$oid} handleCheck={handleCheck} checked={todo.completed} />
                <h4 className={cn("text-gray-200 flex-1", todo.completed && "line-through text-gray-200/50")}>{todo.body}</h4>
                <X className={cn("text-white opacity-0 group-hover/todo:opacity-100")} onClick={handleDelete} />
            </div>
        </article>
    )
}

export default Todo