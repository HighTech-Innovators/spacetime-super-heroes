import http from 'k6/http';
import { check } from 'k6';

const host = __ENV.K6_HOST || "http://localhost:8080"

export function randomFight() {
    const json_post_header = {
        headers: {
            'Content-Type': 'application/json',
        },
    };
    
    var fight_response = http.post(host + "/random_fight", json_post_header);
    //console.log(JSON.stringify(fight_response));
    check(fight_response, {
        'fight result is 200': (r) => r.status === 200
    })
    var fight_result = JSON.parse(fight_response.body);
    return fight_result
}

export function randomFights(count) {
    const json_post_header = {
        headers: {
            'Content-Type': 'application/json',
        },
    };
    
    var fight_response = http.post(host + "/random_fights/" + count, json_post_header);
    //console.log(JSON.stringify(fight_response));
    check(fight_response, {
        'fight result is 200': (r) => r.status === 200
    })
    var fight_result = JSON.parse(fight_response.body);
    return fight_result
}