import { isNil } from "../utils";
import { ResourceChange } from "./type";

export class ResourceService {
    private map_ = new Map<number, Uint8Array>()
    private pngBase64Map_ = new Map<number, string>()

    public applyResourcesChanged(changes: ResourceChange[]) {
        for (const { id, buf } of changes) {
            if (isNil(buf)) {
                this.map_.delete(id)
                this.pngBase64Map_.delete(id)
            } else {
                const bytes = new Uint8Array(buf.length)
                for (let i = 0; i < buf.length; i++) {
                    bytes[i] = buf[i]
                }

                this.map_.set(id, bytes);
            }
        }
    }

    public getAsPngBase64(id: number): string {
        if (this.pngBase64Map_.has(id)) {
            return this.pngBase64Map_.get(id)!
        }

        const buf = this.map_.get(id)
        if (buf) {
            const encoder = new TextDecoder()
            const b64encoded = encoder.decode(buf)
            this.pngBase64Map_.set(id, "data:image/png;base64," + b64encoded)
        } else {
            this.pngBase64Map_.set(id, "")
        }
        return this.pngBase64Map_.get(id)!
    }
}

export const resourceService = new ResourceService()