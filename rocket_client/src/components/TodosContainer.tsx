import { ComponentProps } from "react"
import cn from "../utils/cn"

const TodosContainer = ({ children, className, ...props }: ComponentProps<'section'>) => {
    return (
        <section className={cn("p-2 space-y-2", className)} {...props}>
            {children}
        </section>
    )
}

export default TodosContainer