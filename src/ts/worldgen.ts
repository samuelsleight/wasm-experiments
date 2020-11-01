import {spawn, Pool, Worker} from "threads"

export class WorldgenCallback {
    constructor() {
        this.pool = Pool(() => spawn(new Worker("./workers/worldgen")), 8);
    }

    does_this_work(num: number) {
        return this.pool.queue(worker => worker(num));
    }

    pool: Pool<any>
}
