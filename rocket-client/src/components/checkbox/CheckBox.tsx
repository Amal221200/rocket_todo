import { Check } from "lucide-react"
import { memo, useId } from "react"
import cn from "../../utils/cn";

const CheckBox = memo(function CheckBox({ checked, handleCheck }: { checked: boolean, handleCheck: () => void }) {
    const uniqueId = useId();

    return (
        <label htmlFor={uniqueId} className="group/container cursor-pointer">
            <div className="border-2 group-hover/container:bg-gray-400 rounded-full p-[1px]">
                <input onInput={() => handleCheck()} id={uniqueId} type="checkbox" className="hidden peer/check" defaultChecked={checked} />
                <Check className={cn("text-white opacity-0", checked && "opacity-100")} size={12} />
            </div>
        </label>
    )
})

export default CheckBox