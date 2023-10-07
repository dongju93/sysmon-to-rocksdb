const { fetchDataBasedOnTime } = require("../fetchData"); // Import necessary functions

async function eventList(parent, { filter, pagination = {} }, context, info) {
    const { datetime } = filter;
    const { start: startDate, end: endDate } = datetime;

    // console.log("Start Date:", startDate);
    // console.log("End Date:", endDate);
    // console.log("Pagination:", pagination);

    // console.log("Start Date Type:", typeof startDate);
    // console.log("End Date Type:", typeof endDate);

    const processCreateEvents = await fetchDataBasedOnTime(
        "Process Create",
        startDate,
        endDate,
        filter
    );
    const regValueSetEvents = await fetchDataBasedOnTime(
        "Registry value set",
        startDate,
        endDate,
        filter
    );
    const networkConnectionEvents = await fetchDataBasedOnTime(
        "Network connection detected",
        startDate,
        endDate,
        filter
    );

    // Combine the results. You can decide the order or apply further logic here.
    const combinedResults = [
        ...processCreateEvents,
        ...regValueSetEvents,
        ...networkConnectionEvents,
    ].sort((a, b) => a.savedtimeEpoch - b.savedtimeEpoch);

    const sliceStart = pagination.after
        ? combinedResults.findIndex(
              (item) => item.savedtimeEpoch === pagination.after
          ) + 1
        : 0;

    const sliceEnd = pagination.before
        ? combinedResults.findIndex(
              (item) => item.savedtimeEpoch === pagination.before
          )
        : undefined;

    combinedResults.sort(
        (a, b) => Number(a.savedtimeEpoch) - Number(b.savedtimeEpoch)
    );

    const slicedResults = combinedResults.slice(sliceStart, sliceEnd);

    const edges = slicedResults.map((item) => ({
        cursor: item.savedtimeEpoch,
        node: item,
    }));

    const hasNextPage =
        pagination &&
        pagination.after &&
        combinedResults.length > (pagination.after || 0);
    const hasPreviousPage =
        pagination &&
        pagination.before &&
        combinedResults.length < (pagination.before || 0);

    return {
        edges,
        pageInfo: {
            endCursor: edges[edges.length - 1]?.cursor || null,
            hasNextPage,
            hasPreviousPage,
        },
        totalCount: combinedResults.length,
    };
}

function resolveType(obj) {
    if (obj.event_type) {
        return "RegValueSetEve";
    }
    if (obj.protocol) {
        return "NetworkConnectionEve";
    }
    return "ProcessCreateEve";
}

module.exports = {
    eventList,
    resolveType,
};