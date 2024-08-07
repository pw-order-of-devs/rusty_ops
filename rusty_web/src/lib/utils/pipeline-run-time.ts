import moment from 'moment/moment';
import type { Pipeline } from '$lib/domain/pipeline';

export const updateRunTime = (entry: Pipeline | undefined) => {
	if (entry === undefined) {
		return '';
	}

	let executionTime = '';
	let duration = moment.duration(
		moment(entry.endDate).valueOf() - moment(entry.startDate).valueOf()
	);
	if (duration.isValid()) {
		executionTime = 'Executed in ';
		executionTime = buildRunTime(executionTime, duration);
		executionTime += ' @ ' + moment(entry.endDate).fromNow(false);
	} else {
		duration = moment.duration(moment.now().valueOf() - moment(entry.startDate).valueOf());
		if (duration.isValid()) {
			executionTime = 'Running for ';
			executionTime = buildRunTime(executionTime, duration);
		} else {
			executionTime = 'Created ' + moment(entry.registerDate).fromNow();
		}
	}
	return executionTime;
};

const buildRunTime = (executionTime: string, duration: moment.Duration) => {
	if (duration.days() > 0) executionTime += duration.days().toString() + 'd ';
	if (duration.hours() > 0) executionTime += duration.hours().toString() + 'h ';
	if (duration.minutes() > 0) executionTime += duration.minutes().toString() + 'd ';
	if (duration.seconds() > 0) executionTime += duration.seconds().toString() + 's ';
	executionTime += duration.milliseconds().toString() + 'ms';
	return executionTime;
};
