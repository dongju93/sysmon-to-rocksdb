import type { NextApiRequest, NextApiResponse } from "next";
import axios from "axios";

interface NetworkConnectionNode {
    agent_name: string;
    agent_id: string;
    event_action: string;
    utc_time: string;
    process_guid: string;
    process_id: number;
    image: string;
    user: string;
    protocol: string;
    initiated: boolean;
    source_is_ipv6: boolean;
    source_ip: string;
    source_hostname: string;
    source_port: number;
    destination_is_ipv6: boolean;
    destination_ip: string;
    destination_hostname: string;
    destination_port: number;
    destination_port_name: string;
}

interface NetworkConnectionEdge {
    node: NetworkConnectionNode;
}

interface PageInfo {
    startCursor: string;
    endCursor: string;
    hasNextPage: boolean;
    hasPreviousPage: boolean;
}

interface NetworkConnectionEve {
    totalCount: number;
    pageInfo: PageInfo;
    edges: NetworkConnectionEdge[];
}

interface GraphQLData {
    NetworkConnectionEve: NetworkConnectionEve;
}

interface GraphQLResponse {
    data: GraphQLData;
    errors?: any[];
}

type GraphQLQuery = {
    query: string;
    variables: {
        [key: string]: any;
    };
};

export default async function POST(
    req: NextApiRequest,
    res: NextApiResponse<GraphQLResponse | { message: string }>
) {
    if (req.method === "POST") {
        console.log("Request body:", req.body);
        const { startTime, endTime, perPage, before, selectedOption } = req.body;

        const graphqlQuery: GraphQLQuery = {
            query: `
              query getRawEvents($start: String!, $end: String!, $last: Int, $rawEvents: String!) {
                  $rawEvents(
                      filter: {
                          datetime: {
                              start: $start,
                              end: $end
                          }
                      }
                      pagination: {
                          last: $last,
                      }
                  ) {
                      totalCount
                      pageInfo {
                          endCursor
                          hasNextPage
                          hasPreviousPage
                      }
                      edges {
                          cursor
                          node {
                              agent_name
                              agent_id
                              event_action
                              utc_time
                              process_guid
                              process_id
                              image
                              user
                              protocol
                              initiated
                              source_is_ipv6
                              source_ip
                              source_hostname
                              source_port
                              destination_is_ipv6
                              destination_ip
                              destination_hostname
                              destination_port
                              destination_port_name
                          }
                      }
                  }
              }
          `,
            variables: {
                start: startTime,
                end: endTime,
                last: perPage,
                rawEvents: selectedOption
            },
        };

        const requestConfig = {
            headers: { "Content-Type": "application/json" },
            method: "POST",
            body: graphqlQuery,
        };

        console.log("Request Method:", requestConfig.method);
        console.log("Request Headers:", requestConfig.headers);
        console.log("Request Body:", requestConfig.body);

        try {
            const response = await axios.post(
                "http://localhost:4000/",
                graphqlQuery,
                {
                    headers: requestConfig.headers,
                }
            );

            res.status(200).json(response.data);
        } catch (error) {
            console.error(error);
            res.status(500).json({ message: "Error fetching data" });
        }
    } else {
        console.error(res);
        // res.status(405).end();
    }
}