import { ComponentProps } from "react"
import cn from "../utils/cn"
import { SortableContext, verticalListSortingStrategy } from "@dnd-kit/sortable"
import useTodo from "../hooks/useTodo"

const TodosContainer = ({ children, className, ...props }: ComponentProps<'section'>) => {
    const { todos } = useTodo()
    if (!todos)
        return
    return (
        <section className={cn("p-2 space-y-2", className)} {...props}>
            <SortableContext items={todos.map((todo) => ({ id: todo._id.$oid }))} strategy={verticalListSortingStrategy}>
                {children}
            </SortableContext>
        </section>
    )
}

export default TodosContainer