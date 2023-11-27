import { KV, useEntity } from "@/utils/useEntity";

export default function Entity({ id }: { id: string }) {
    let entity = useEntity(id)
    return (
        <div className="mt-4 shadow-md rounded-md">
            <h1 className="text-2xl font-bold mb-4">{entity?.name}</h1>
            {attrisView(entity.attris)}
        </div>
    )
}

function attrisView(kv: KV) {
    return (
        <div className="grid grid-cols-1 justify-items-start">
            {kv && Object.keys(kv).map((key) => {
                return (
                    <div key={key} className="text-left">
                        <div className="text-lg font-semibold">{`${key}:`}</div>
                        <div className="text-base ml-4">{
                            (typeof kv[key] == 'string') ?
                                kv[key] as string :
                                attrisView(kv[key] as KV)
                        }</div>
                    </div>
                );
            })}
        </div>
    )
}